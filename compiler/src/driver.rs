//! Kip driver
//!

use crate::cli::Options;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::scopechk::ScopeChecker;
use crate::source::Source;

use anyhow::{bail, Context, Result};
use atty::Stream;

use std::fs;
use std::io::{self, Read};

pub fn run(options: Options) -> Result<()> {
    let source = if let Some(source_path) = options.input {
        // user provided a path to a source file
        let name = source_path.to_string_lossy().into_owned();
        let contents = fs::read_to_string(&source_path)
            .with_context(|| format!("Failed to read code from {}", name))?;
        Source { contents, name }
    } else if atty::isnt(Stream::Stdin) {
        // the user (presumably) passed a source file through stdin
        println!("Reading source file from stdin");
        let mut source = Source::new(String::new(), "<stdin>");
        io::stdin().read_to_string(&mut source.contents)?;
        source
    } else {
        bail!("No input");
    };

    let mut lexer = Lexer::new(&source);
    let tokens = lexer.lex();
    let mut parser = Parser::new(tokens, &source);

    let mut parse_errors = vec![];
    let module = parser
        .parse()
        .into_iter()
        .filter_map(|stmt| stmt.map_err(|e| parse_errors.push(e)).ok())
        .collect();

    for err in &parse_errors {
        println!("Syntax Error: {}", err);
    }

    if !parse_errors.is_empty() {
        bail!("Failed to parse file");
    }

    let mut scopechk = ScopeChecker::new();
    scopechk.check(&module);

    Ok(())
}

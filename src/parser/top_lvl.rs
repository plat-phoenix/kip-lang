use log::error;

use crate::ast::visit::Visitor;
use crate::lexer::Token;
use crate::parser::Parser;
use crate::typechk::TypeChecker;

impl<'a> Parser<'a> {
    fn handle_func(&mut self) {
        if let Ok(func) = self.parse_func() {
            // trace!("parsed a function definition: {:#?}", func);
            let mut typechk = TypeChecker::new(self.input(), &self.sym_tbl);
            typechk.visit_func(&func);
            for err in typechk.errors() {
                err.display().unwrap();
            }
        } else {
            self.eat();
        }
    }

    fn handle_extern(&mut self) {
        if self.parse_extern().is_err() {
            self.eat();
        }
    }

    fn handle_top_lvl_expr(&mut self) {
        if self.parse_top_lvl_expr().is_err() {
            self.eat();
        }
    }

    fn handle_impt_stmt(&mut self) {
        if self.parse_impt_stmt().is_err() {
            self.eat();
        }
    }
    pub fn parse(&mut self) {
        loop {
            match self.peek() {
                Token::Eof => break,
                Token::Impt => {
                    self.eat();
                    self.handle_impt_stmt();
                }
                Token::Func => {
                    self.eat();
                    self.handle_func();
                }
                Token::Extern => {
                    self.eat();
                    self.handle_extern();
                }
                _ => self.handle_top_lvl_expr(),
            }
        }

        for err in &self.errors {
            error!(target: "parser", "{}", err);
        }
    }
}

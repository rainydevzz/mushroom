use crate::frontend::{self, lexer::TokenType, ast::{MmbrCall, self}};

use super::ast::VarExpr;

pub struct Parser {
    pub tokens: Vec<frontend::lexer::Token>
}

impl Parser {
    pub fn new(tokens: Vec<frontend::lexer::Token>) -> Self {
        Parser { tokens }
    }

    pub fn parse(&mut self) -> Vec<ast::Stmt> {
        let mut ast_result: Vec<frontend::ast::Stmt> = Vec::new();
        while self.tokens.len() > 0 {
            let mut temp = Vec::new();
            let tok = self.tokens.remove(0);
            match tok.kind {
                frontend::lexer::TokenType::Let => {
                    temp.push(self.tokens.remove(0));
                    if matches!(temp[0].kind, frontend::lexer::TokenType::Identifier) {
                        self.tokens.remove(0);
                        temp.push(self.tokens.remove(0));
                        if matches!(temp[1].kind, frontend::lexer::TokenType::String) {
                            ast_result.push(frontend::ast::Stmt::VarExpr(VarExpr {identifier: temp[0].value.clone(), value: frontend::ast::VarTypes::Str(temp[1].value.clone())}));
                        } else if matches!(temp[1].kind, frontend::lexer::TokenType::Number) {
                            ast_result.push(frontend::ast::Stmt::VarExpr(VarExpr {identifier: temp[0].value.clone(), value: frontend::ast::VarTypes::Num(temp[1].value.clone().parse::<i32>().unwrap())}));
                        } else {
                            panic!("only integers and strings are supported.");
                        }
                    }
                }

                frontend::lexer::TokenType::Identifier => {
                    temp.push(tok);
                    if matches!(self.tokens.remove(0).kind, frontend::lexer::TokenType::LParen) {
                        temp.push(self.tokens.remove(0));
                        while !matches!(temp[temp.len() - 1].kind, TokenType::RParen) {
                            if self.tokens.len() == 0 {
                                panic!("expected ')'");
                            }
                            temp.push(self.tokens.remove(0));
                        }
                        temp.pop();
                        ast_result.push(frontend::ast::Stmt::MmbrCall(MmbrCall {member: temp.remove(0).value, params: temp}))
                    }
                }

                _ => todo!()
            }
        }
        ast_result
    }
}
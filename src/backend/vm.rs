use std::collections::HashMap;
use crate::frontend::{self, parser::Parser, ast::VarTypes};

#[derive(Debug)]
pub struct Environment {
    variables: HashMap<String, frontend::ast::VarTypes>
}

pub struct VM {
    env: Environment
}

impl VM {
    pub fn new() -> Self {
        VM { env: Environment { variables: HashMap::new() } }
    }

    pub fn alloc(&mut self, k: String, v: frontend::ast::VarTypes) {
        self.env.variables.insert(k, v);
    }

    pub fn run(&mut self, mut parser: Parser) {
        let calls = parser.parse();
        for c in calls.iter() {
            match c {
                frontend::ast::Stmt::VarExpr(c) => {
                    self.alloc(c.identifier.clone(), c.value.clone());
                    println!("{:?}", &self.env);
                }
                frontend::ast::Stmt::MmbrCall(c) => {
                    if c.member == "print".to_owned() {
                        for p in c.params.iter() {
                            if let VarTypes::Str(s) = self.env.variables.get(&p.value).unwrap() {
                                println!("{:?}", s);
                            } else if let VarTypes::Num(s) = self.env.variables.get(&p.value).unwrap() {
                                println!("{:?}", s);
                            } else {
                                panic!("param doesn't exist");
                            }
                        }
                    }
                }
            }
        }
        println!("{:?}", &calls);
    }
}
use super::lexer::Token;

#[derive(Debug)]
pub enum Stmt {
    VarExpr(VarExpr),
    MmbrCall(MmbrCall)
}

#[derive(Debug, Clone)]
pub enum VarTypes {
    Str(String),
    Num(i32)
}

#[derive(Debug)]
pub struct VarExpr {
    pub identifier: String,
    pub value: VarTypes
}

#[derive(Debug)]
pub struct MmbrCall {
    pub member: String,
    pub params: Vec<Token>
}
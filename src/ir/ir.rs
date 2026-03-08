use std::collections::HashMap;

use boa_ast::expression::Identifier;

#[derive(Clone, Debug)]
pub enum IRExpr {
    Const(i8),
    Add(Box<IRExpr>, Box<IRExpr>),
    Sub(Box<IRExpr>, Box<IRExpr>),
    Mul(Box<IRExpr>, Box<IRExpr>),
    Div(Box<IRExpr>, Box<IRExpr>),
    Id(Identifier),
    Call {
        id: Identifier,
        args: Vec<IRExpr>,
    },
    Fetch {
        address: Box<IRExpr>,
        index: usize,
    },
}

#[derive(Clone, Debug)]
pub struct IRVarInit {
    pub id: Identifier,
    pub init: Option<IRExpr>,
}

#[derive(Clone, Debug)]
pub enum IRStmt {
    Noop,
    VariableDefine {
        vars: Vec<IRVarInit>,
    },
    Assign {
        id: Identifier,
        value: IRExpr,
    },
    Call {
        id: Identifier,
        args: Vec<IRExpr>,
    },
    While {
        condition: IRExpr,
        body: Vec<IRStmt>,
    },
    If {
        condition: IRExpr,
        body: Vec<IRStmt>,
        else_body: Option<Vec<IRStmt>>,
    },
    Block {
        body: Vec<IRStmt>,
    },
}

#[derive(Clone, Debug)]
pub struct IRFunc {
    pub args: Vec<Identifier>,
    pub code: Vec<IRStmt>,
    pub result: Option<IRExpr>,
}

#[derive(Clone, Debug)]
pub struct IR {
    pub main: Vec<IRStmt>,
    pub funcs: HashMap<Identifier, IRFunc>,
}

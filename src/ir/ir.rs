use std::collections::HashMap;

use boa_ast::expression::Identifier;

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

pub struct IRVarInit {
    pub id: Identifier,
    pub init: Option<IRExpr>,
}

pub enum IRStmt {
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

pub struct IRFunc {
    args: Vec<String>,
    code: Vec<IRStmt>,
    result: Option<IRExpr>,
}

pub struct IR {
    main: Vec<IRStmt>,
    funcs: HashMap<String, IRFunc>,
}

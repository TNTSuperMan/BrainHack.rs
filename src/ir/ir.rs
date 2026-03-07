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

pub enum IRStmt {
    VariableDefine {
        id: Identifier,
        init: Option<IRExpr>,
    },
    Assign {
        id: String,
        value: IRExpr,
    },
    Call {
        id: Identifier,
        args: Vec<IRExpr>,
    },
    While {
        condition: IRExpr,
        body: Box<IRStmt>,
    },
    If {
        condition: IRExpr,
        body: Box<IRStmt>,
        else_body: Option<Box<IRStmt>>,
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

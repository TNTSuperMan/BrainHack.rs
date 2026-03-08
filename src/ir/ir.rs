use std::collections::HashMap;

use boa_interner::Sym;

#[derive(Clone, Debug)]
pub enum IRExpr {
    Const(i8),
    Add(Box<IRExpr>, Box<IRExpr>),
    Sub(Box<IRExpr>, Box<IRExpr>),
    Mul(Box<IRExpr>, Box<IRExpr>),
    Div(Box<IRExpr>, Box<IRExpr>),
    Id {
        id: Sym,
        last_use: bool,
    },
    Call {
        id: Sym,
        args: Vec<IRExpr>,
    },
    Fetch {
        address: Box<IRExpr>,
        index: usize,
    },
}

#[derive(Clone, Debug)]
pub struct IRVarInit {
    pub id: Sym,
    pub init: Option<IRExpr>,
}

#[derive(Clone, Debug)]
pub enum IRStmt {
    Noop,
    VariableDefine {
        vars: Vec<IRVarInit>,
    },
    Assign {
        id: Sym,
        value: IRExpr,
    },
    Call {
        id: Sym,
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
    Out {
        val: IRExpr,
    },
}

#[derive(Clone, Debug)]
pub struct IRFunc {
    pub args: Vec<Sym>,
    pub code: Vec<IRStmt>,
    pub result: Option<IRExpr>,
}

#[derive(Clone, Debug)]
pub struct IR {
    pub main: Vec<IRStmt>,
    pub funcs: HashMap<Sym, IRFunc>,
}

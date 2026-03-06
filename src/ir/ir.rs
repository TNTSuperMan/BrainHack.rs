use std::collections::HashMap;

pub enum IRExpr {
    Const(i8),
    Add(Box<IRExpr>, Box<IRExpr>),
    Sub(Box<IRExpr>, Box<IRExpr>),
    Mul(Box<IRExpr>, Box<IRExpr>),
    Div(Box<IRExpr>, Box<IRExpr>),
    Id(String),
    Call {
        name: String,
        args: Vec<IRExpr>,
    },
    Fetch {
        address: Box<IRExpr>,
        index: usize,
    },
}

pub enum IRStatement {
    VariableDefine {
        id: String,
        init: Option<IRExpr>,
    },
    Assign {
        id: String,
        value: IRExpr,
    },
    Call {
        name: String,
        args: Vec<IRExpr>,
    },
    While {
        condition: IRExpr,
        body: Box<IRStatement>,
    },
    If {
        condition: IRExpr,
        body: Box<IRStatement>,
        else_body: Option<Box<IRStatement>>,
    },
    Block {
        body: Vec<IRStatement>,
    },
}

pub struct IRFunc {
    args: Vec<String>,
    code: Vec<IRStatement>,
    result: Option<IRExpr>,
}

pub struct IR {
    main: Vec<IRStatement>,
    funcs: HashMap<String, IRFunc>,
}

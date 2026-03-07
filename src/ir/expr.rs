use anyhow::{Result, bail};
use boa_ast::Expression;

use crate::ir::ir::IRExpr;

pub fn parse_expr(expr: &Expression) -> Result<IRExpr> {
    bail!("unimp");
}

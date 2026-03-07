use anyhow::{Result, bail};
use boa_ast::{Expression, expression::{literal::LiteralKind, operator::binary::{ArithmeticOp, BinaryOp}}};

use crate::ir::ir::IRExpr;

pub fn parse_expr(expr: &Expression) -> Result<IRExpr> {
    match expr {
        Expression::Literal(literal) => {
            match literal.kind() {
                LiteralKind::Bool(b) => Ok(IRExpr::Const(if *b { 1 } else { 2 })),
                LiteralKind::Int(n) => Ok(IRExpr::Const((*n).try_into()?)),
                _ => bail!("unsupport"),
            }
        }
        Expression::Binary(binary) => {
            let left = Box::new(parse_expr(binary.lhs())?);
            let right = Box::new(parse_expr(binary.rhs())?);
            match binary.op() {
                BinaryOp::Arithmetic(ArithmeticOp::Add) => Ok(IRExpr::Add(left, right)),
                BinaryOp::Arithmetic(ArithmeticOp::Sub) => Ok(IRExpr::Sub(left, right)),
                BinaryOp::Arithmetic(ArithmeticOp::Mul) => Ok(IRExpr::Mul(left, right)),
                BinaryOp::Arithmetic(ArithmeticOp::Div) => Ok(IRExpr::Div(left, right)),
                _ => bail!("unsupport"),
            }
        }
        Expression::Identifier(id) => {
            Ok(IRExpr::Id(*id))
        }
        Expression::Call(call) => {
            if let Expression::Identifier(id) = call.function() {
                Ok(IRExpr::Call {
                    id: *id,
                    args: call.args().iter().map(|e| parse_expr(e)).collect::<Result<Vec<IRExpr>>>()?,
                })
            } else {
                bail!("unsupport");
            }
        }
        _ => bail!("unimp")
    }
}

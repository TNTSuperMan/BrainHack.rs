use anyhow::{Result, bail};
use boa_ast::{Expression, expression::{literal::LiteralKind, operator::{binary::{ArithmeticOp, BinaryOp, RelationalOp}, unary::UnaryOp}}};

use crate::ir::{ctx::ParserContext, ir::IRExpr};

pub fn parse_expr(ctx: &mut ParserContext, expr: &Expression) -> Result<IRExpr> {
    match expr {
        Expression::Literal(literal) => {
            match literal.kind() {
                LiteralKind::Bool(b) => Ok(IRExpr::Const(if *b { 1 } else { 2 })),
                LiteralKind::Int(n) => Ok(IRExpr::Const((*n).try_into()?)),
                _ => bail!("unsupport"),
            }
        }
        Expression::Binary(binary) => {
            let left = Box::new(parse_expr(ctx, binary.lhs())?);
            let right = Box::new(parse_expr(ctx, binary.rhs())?);
            Ok(match binary.op() {
                BinaryOp::Arithmetic(ArithmeticOp::Add) => IRExpr::Add(left, right),
                BinaryOp::Arithmetic(ArithmeticOp::Sub) => IRExpr::Sub(left, right),
                BinaryOp::Arithmetic(ArithmeticOp::Mul) => IRExpr::Mul(left, right),
                BinaryOp::Arithmetic(ArithmeticOp::Div) => IRExpr::Div(left, right),
                BinaryOp::Relational(RelationalOp::Equal) |
                BinaryOp::Relational(RelationalOp::StrictEqual) => IRExpr::BoolNot(Box::new(IRExpr::Sub(left, right))),
                BinaryOp::Relational(RelationalOp::NotEqual) |
                BinaryOp::Relational(RelationalOp::StrictNotEqual) => IRExpr::Sub(left, right),
                BinaryOp::Relational(RelationalOp::GreaterThan) => IRExpr::Gt(left, right),
                BinaryOp::Relational(RelationalOp::LessThan) => IRExpr::Gt(right, left),
                BinaryOp::Relational(RelationalOp::GreaterThanOrEqual) => IRExpr::BoolNot(Box::new(IRExpr::Gt(right, left))),
                BinaryOp::Relational(RelationalOp::LessThanOrEqual) => IRExpr::BoolNot(Box::new(IRExpr::Gt(left, right))),
                _ => bail!("unsupport"),
            })
        }
        Expression::Unary(unary) => {
            let target = Box::new(parse_expr(ctx, unary.target())?);
            Ok(match unary.op() {
                UnaryOp::Plus => *target,
                UnaryOp::Minus => IRExpr::Sub(Box::new(IRExpr::Const(0)), target),
                UnaryOp::Not => IRExpr::BoolNot(target),
                _ => bail!("unsupport"),
            })
        }
        Expression::Identifier(id) => {
            Ok(IRExpr::Id { id: id.sym(), last_use: false })
        }
        Expression::Call(call) => {
            if let Expression::Identifier(id) = call.function() {
                if id.sym() == ctx.interner.get("input").unwrap() {
                    if call.args().len() != 0 {
                        bail!("Input function argument size mismatch");
                    }
                    Ok(IRExpr::Input)
                } else {
                    call.args().iter().map(|e| parse_expr(ctx, e)).collect::<Result<Vec<IRExpr>>>().map(|args| {
                        IRExpr::Call {
                            id: id.sym(),
                            args,
                        }
                    })
                }
            } else {
                bail!("unsupport");
            }
        }
        _ => bail!("unimp")
    }
}

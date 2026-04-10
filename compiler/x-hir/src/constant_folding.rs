//! HIR 常量折叠优化预处理
//!
//! 在 HIR 层进行简单的常量折叠，提前计算编译期可确定的表达式：
//! 1. 折叠字面量的二元运算
//! 2. 折叠字面量的一元运算
//! 3. 当 if 条件是常量时折叠分支
//! 4. 简化编译期可确定的表达式

use crate::{HirBinaryOp, HirExpression, HirLiteral, HirUnaryOp};

/// 尝试对 HIR 表达式进行常量折叠
///
/// 如果整个表达式可以折叠为单个常量，返回折叠后的字面量；否则返回 None
pub fn try_constant_fold(expr: &HirExpression) -> Option<HirLiteral> {
    match expr {
        HirExpression::Literal(lit) => Some(lit.clone()),

        HirExpression::Binary(op, left, right) => {
            let left_lit = try_constant_fold(left)?;
            let right_lit = try_constant_fold(right)?;

            fold_binary_op(op.clone(), left_lit, right_lit)
        }

        HirExpression::Unary(op, expr) => {
            let lit = try_constant_fold(expr)?;
            fold_unary_op(op.clone(), lit)
        }

        HirExpression::Cast(expr, _ty) => {
            // 如果被转换的表达式是常量，保留它
            try_constant_fold(expr)
        }

        _ => None,
    }
}

/// 折叠二元运算，如果两个操作数都是常量
fn fold_binary_op(op: HirBinaryOp, left: HirLiteral, right: HirLiteral) -> Option<HirLiteral> {
    match (left, right) {
        (HirLiteral::Integer(l), HirLiteral::Integer(r)) => {
            let result = match op {
                HirBinaryOp::Add => HirLiteral::Integer(l.wrapping_add(r)),
                HirBinaryOp::Sub => HirLiteral::Integer(l.wrapping_sub(r)),
                HirBinaryOp::Mul => HirLiteral::Integer(l.wrapping_mul(r)),
                HirBinaryOp::Div if r != 0 => HirLiteral::Integer(l / r),
                HirBinaryOp::Mod if r != 0 => HirLiteral::Integer(l % r),
                HirBinaryOp::Equal => HirLiteral::Boolean(l == r),
                HirBinaryOp::NotEqual => HirLiteral::Boolean(l != r),
                HirBinaryOp::Less => HirLiteral::Boolean(l < r),
                HirBinaryOp::LessEqual => HirLiteral::Boolean(l <= r),
                HirBinaryOp::Greater => HirLiteral::Boolean(l > r),
                HirBinaryOp::GreaterEqual => HirLiteral::Boolean(l >= r),
                HirBinaryOp::BitAnd => HirLiteral::Integer(l & r),
                HirBinaryOp::BitOr => HirLiteral::Integer(l | r),
                HirBinaryOp::BitXor => HirLiteral::Integer(l ^ r),
                HirBinaryOp::LeftShift if (0_i64..64).contains(&r) => HirLiteral::Integer(l << r),
                HirBinaryOp::RightShift if (0_i64..64).contains(&r) => HirLiteral::Integer(l >> r),
                _ => return None,
            };
            Some(result)
        }

        (HirLiteral::Float(l), HirLiteral::Float(r)) => {
            let result = match op {
                HirBinaryOp::Add => HirLiteral::Float(l + r),
                HirBinaryOp::Sub => HirLiteral::Float(l - r),
                HirBinaryOp::Mul => HirLiteral::Float(l * r),
                HirBinaryOp::Div => HirLiteral::Float(l / r),
                HirBinaryOp::Equal => HirLiteral::Boolean(l == r),
                HirBinaryOp::NotEqual => HirLiteral::Boolean(l != r),
                HirBinaryOp::Less => HirLiteral::Boolean(l < r),
                HirBinaryOp::LessEqual => HirLiteral::Boolean(l <= r),
                HirBinaryOp::Greater => HirLiteral::Boolean(l > r),
                HirBinaryOp::GreaterEqual => HirLiteral::Boolean(l >= r),
                _ => return None,
            };
            Some(result)
        }

        (HirLiteral::Boolean(l), HirLiteral::Boolean(r)) => {
            let result = match op {
                HirBinaryOp::And => HirLiteral::Boolean(l && r),
                HirBinaryOp::Or => HirLiteral::Boolean(l || r),
                HirBinaryOp::Equal => HirLiteral::Boolean(l == r),
                HirBinaryOp::NotEqual => HirLiteral::Boolean(l != r),
                _ => return None,
            };
            Some(result)
        }

        (HirLiteral::String(mut l), HirLiteral::String(r)) => {
            if op == HirBinaryOp::Concat {
                l.push_str(&r);
                Some(HirLiteral::String(l))
            } else {
                None
            }
        }

        _ => None,
    }
}

/// 折叠一元运算
fn fold_unary_op(op: HirUnaryOp, lit: HirLiteral) -> Option<HirLiteral> {
    match (op, lit) {
        (HirUnaryOp::Negate, HirLiteral::Integer(i)) => Some(HirLiteral::Integer(-i)),
        (HirUnaryOp::Negate, HirLiteral::Float(f)) => Some(HirLiteral::Float(-f)),
        (HirUnaryOp::Not, HirLiteral::Boolean(b)) => Some(HirLiteral::Boolean(!b)),
        (HirUnaryOp::BitNot, HirLiteral::Integer(i)) => Some(HirLiteral::Integer(!i)),
        _ => None,
    }
}

/// 对整个表达式递归进行常量折叠，返回折叠后的新表达式
pub fn constant_fold(expr: HirExpression) -> HirExpression {
    match expr {
        // 先递归折叠子表达式
        HirExpression::Binary(op, left, right) => {
            let folded_left = Box::new(constant_fold(*left));
            let folded_right = Box::new(constant_fold(*right));

            if let Some(result) = try_constant_fold(&HirExpression::Binary(
                op.clone(),
                folded_left.clone(),
                folded_right.clone(),
            )) {
                HirExpression::Literal(result)
            } else {
                HirExpression::Binary(op, folded_left, folded_right)
            }
        }

        HirExpression::Unary(op, expr) => {
            let folded = Box::new(constant_fold(*expr));

            if let Some(result) =
                try_constant_fold(&HirExpression::Unary(op.clone(), folded.clone()))
            {
                HirExpression::Literal(result)
            } else {
                HirExpression::Unary(op, folded)
            }
        }

        HirExpression::Cast(expr, ty) => {
            let folded = Box::new(constant_fold(*expr));

            if let Some(result) =
                try_constant_fold(&HirExpression::Cast(folded.clone(), ty.clone()))
            {
                HirExpression::Literal(result)
            } else {
                HirExpression::Cast(folded, ty)
            }
        }

        HirExpression::If(cond, then, else_) => {
            let folded_cond = Box::new(constant_fold(*cond));

            // 如果条件是常量布尔值，直接返回对应的分支
            if let Some(HirLiteral::Boolean(cond_val)) = try_constant_fold(&folded_cond) {
                if cond_val {
                    constant_fold(*then)
                } else {
                    constant_fold(*else_)
                }
            } else {
                HirExpression::If(
                    folded_cond,
                    Box::new(constant_fold(*then)),
                    Box::new(constant_fold(*else_)),
                )
            }
        }

        HirExpression::Call(func, args) => HirExpression::Call(
            Box::new(constant_fold(*func)),
            args.into_iter().map(constant_fold).collect(),
        ),

        HirExpression::Member(expr, name) => {
            HirExpression::Member(Box::new(constant_fold(*expr)), name)
        }

        HirExpression::Assign(left, right) => HirExpression::Assign(
            Box::new(constant_fold(*left)),
            Box::new(constant_fold(*right)),
        ),

        HirExpression::Array(items) => {
            HirExpression::Array(items.into_iter().map(constant_fold).collect())
        }

        HirExpression::Tuple(items) => {
            HirExpression::Tuple(items.into_iter().map(constant_fold).collect())
        }

        HirExpression::Dictionary(items) => HirExpression::Dictionary(
            items
                .into_iter()
                .map(|(k, v)| (constant_fold(k), constant_fold(v)))
                .collect(),
        ),

        HirExpression::Record(name, items) => HirExpression::Record(
            name,
            items
                .into_iter()
                .map(|(n, v)| (n, constant_fold(v)))
                .collect(),
        ),

        HirExpression::Range(start, end, inclusive) => HirExpression::Range(
            Box::new(constant_fold(*start)),
            Box::new(constant_fold(*end)),
            inclusive,
        ),

        HirExpression::Pipe(expr, stages) => HirExpression::Pipe(
            Box::new(constant_fold(*expr)),
            stages.into_iter().map(constant_fold).collect(),
        ),

        HirExpression::Match(expr, cases) => HirExpression::Match(
            Box::new(constant_fold(*expr)),
            cases
                .into_iter()
                .map(|(pat, guard, block)| (pat, guard.map(|g| Box::new(constant_fold(*g))), block))
                .collect(),
        ),

        HirExpression::Await(expr) => HirExpression::Await(Box::new(constant_fold(*expr))),

        HirExpression::OptionalChain(expr, field) => {
            HirExpression::OptionalChain(Box::new(constant_fold(*expr)), field)
        }

        HirExpression::NullCoalescing(left, right) => HirExpression::NullCoalescing(
            Box::new(constant_fold(*left)),
            Box::new(constant_fold(*right)),
        ),

        HirExpression::TryPropagate(expr) => {
            HirExpression::TryPropagate(Box::new(constant_fold(*expr)))
        }

        HirExpression::Typed(expr, ty) => {
            let folded = constant_fold(*expr);
            if let HirExpression::Literal(lit) = folded {
                HirExpression::Literal(lit)
            } else {
                HirExpression::Typed(Box::new(folded), ty)
            }
        }

        HirExpression::Handle(expr, handlers) => HirExpression::Handle(
            Box::new(constant_fold(*expr)),
            handlers
                .into_iter()
                .map(|(name, handler)| (name, constant_fold(handler)))
                .collect(),
        ),

        // 这些已经是叶子节点或者不需要折叠
        HirExpression::Literal(_)
        | HirExpression::Variable(_)
        | HirExpression::Lambda(_, _)
        | HirExpression::Needs(_)
        | HirExpression::Given(_, _) => expr,

        // 不需要对 Wait 进行特殊折叠，保持原有结构
        HirExpression::Wait(_, _) => expr,
        HirExpression::WhenGuard(_, _) => expr,
        HirExpression::Block(_) => expr,
    }
}

/// 对整个 HIR 模块进行常量折叠预处理
pub fn constant_fold_module(expressions: &mut Vec<HirExpression>) {
    for expr in expressions {
        *expr = constant_fold(expr.clone());
    }
}

//! LIR 层窥孔优化
//!
//! 窥孔优化在基本块级别进行局部优化，处理常见冗余：
//! 1. 消除无用赋值 `x = x`
//! 2. 常量折叠（MIR 层已经做了，这里补漏）
//! 3. 简化条件分支
//! 4. 移除空的基本块
//! 5. 合并连续赋值

use crate::{BinaryOp, Block, Expression, Literal, Statement, Type, UnaryOp};
use std::collections::HashMap;

/// 窥孔优化器
pub struct PeepholeOptimizer;

impl PeepholeOptimizer {
    pub fn new() -> Self {
        Self
    }

    /// 对整个函数的基本块执行窥孔优化
    pub fn optimize_block(&self, block: &mut Block) {
        let mut new_statements = Vec::new();
        let mut value_map: HashMap<String, Expression> = HashMap::new();

        for stmt in block.statements.drain(..) {
            let optimized = self.optimize_statement(stmt, &value_map);

            if let Some(stmt) = optimized {
                // 如果是赋值，更新值映射
                if let Statement::Expression(Expression::Assign(target, value)) = &stmt {
                    if let Expression::Variable(name) = &**target {
                        value_map.insert(name.clone(), (**value).clone());
                    }
                }

                // 移除无用赋值 x = x
                if let Statement::Expression(Expression::Assign(target, value)) = &stmt {
                    if let (Expression::Variable(tn), Expression::Variable(vn)) =
                        (&**target, &**value)
                    {
                        if tn == vn {
                            continue;
                        }
                    }
                }

                new_statements.push(stmt);
            }
        }

        block.statements = new_statements;
    }

    /// 优化单条语句
    fn optimize_statement(
        &self,
        stmt: Statement,
        value_map: &HashMap<String, Expression>,
    ) -> Option<Statement> {
        match stmt {
            Statement::Variable(_) => Some(stmt),
            Statement::Label(_) => Some(stmt),
            Statement::Goto(_) => Some(stmt),
            Statement::If(mut if_stmt) => {
                if_stmt.condition = self.optimize_expression(if_stmt.condition, value_map);
                // 如果条件是常量 true，简化为直接 goto then
                if let Expression::Literal(Literal::Bool(true)) = if_stmt.condition {
                    Some(*if_stmt.then_branch)
                } else if let Expression::Literal(Literal::Bool(false)) = if_stmt.condition {
                    // 如果条件是常量 false，只保留 else
                    if_stmt.else_branch.map(|b| *b)
                } else {
                    Some(Statement::If(if_stmt))
                }
            }
            Statement::Switch(mut switch) => {
                switch.expression = self.optimize_expression(switch.expression, value_map);
                Some(Statement::Switch(switch))
            }
            Statement::Return(expr) => {
                let expr = expr.map(|e| self.optimize_expression(e, value_map));
                Some(Statement::Return(expr))
            }
            Statement::Expression(mut expr) => {
                expr = self.optimize_expression(expr, value_map);
                Some(Statement::Expression(expr))
            }
            // Other statement types are passed through unchanged
            Statement::Declaration(_) => Some(stmt),
            Statement::While(_) => Some(stmt),
            Statement::DoWhile(_) => Some(stmt),
            Statement::For(_) => Some(stmt),
            Statement::Match(_) => Some(stmt),
            Statement::Try(_) => Some(stmt),
            Statement::Break => Some(stmt),
            Statement::Continue => Some(stmt),
            Statement::Empty => Some(stmt),
            Statement::Compound(_) => Some(stmt),
        }
    }

    /// 优化表达式，进行常量折叠和替换
    fn optimize_expression(
        &self,
        expr: Expression,
        value_map: &HashMap<String, Expression>,
    ) -> Expression {
        match expr {
            Expression::Variable(name) => {
                // 如果变量被赋值为常量，直接替换为常量
                if let Some(value) = value_map.get(&name) {
                    match value {
                        Expression::Literal(_) => value.clone(),
                        _ => Expression::Variable(name),
                    }
                } else {
                    Expression::Variable(name)
                }
            }
            Expression::Binary(op, left, right) => {
                let left = self.optimize_expression(*left, value_map);
                let right = self.optimize_expression(*right, value_map);

                // 如果两边都是常量，折叠
                if let (Expression::Literal(left_lit), Expression::Literal(right_lit)) =
                    (&left, &right)
                {
                    if let Some(result) = self.fold_binary(op, left_lit.clone(), right_lit.clone())
                    {
                        return Expression::Literal(result);
                    }
                }

                Expression::Binary(op, Box::new(left), Box::new(right))
            }
            Expression::Unary(op, expr) => {
                let expr = self.optimize_expression(*expr, value_map);

                if let Expression::Literal(lit) = &expr {
                    if let Some(result) = self.fold_unary(op, lit.clone()) {
                        return Expression::Literal(result);
                    }
                }

                Expression::Unary(op, Box::new(expr))
            }
            Expression::Assign(target, value) => {
                let target = self.optimize_expression(*target, value_map);
                let value = self.optimize_expression(*value, value_map);
                Expression::Assign(Box::new(target), Box::new(value))
            }
            Expression::Cast(ty, expr) => {
                let expr = self.optimize_expression(*expr, value_map);
                // 如果是常量，可以尝试转换
                if let Expression::Literal(lit) = &expr {
                    if let Some(result) = self.fold_cast(ty.clone(), lit.clone()) {
                        return Expression::Literal(result);
                    }
                }
                Expression::Cast(ty, Box::new(expr))
            }
            Expression::Call(func, args) => {
                let func = self.optimize_expression(*func, value_map);
                let args = args
                    .into_iter()
                    .map(|arg| self.optimize_expression(arg, value_map))
                    .collect();
                Expression::Call(Box::new(func), args)
            }
            Expression::Member(expr, field) => {
                let expr = self.optimize_expression(*expr, value_map);
                Expression::Member(Box::new(expr), field)
            }
            Expression::Index(array, index) => {
                let array = self.optimize_expression(*array, value_map);
                let index = self.optimize_expression(*index, value_map);
                Expression::Index(Box::new(array), Box::new(index))
            }
            Expression::Dereference(expr) => {
                let expr = self.optimize_expression(*expr, value_map);
                Expression::Dereference(Box::new(expr))
            }
            Expression::Ternary(cond, then, else_) => {
                let cond = self.optimize_expression(*cond, value_map);
                let then = self.optimize_expression(*then, value_map);
                let else_ = self.optimize_expression(*else_, value_map);
                Expression::Ternary(Box::new(cond), Box::new(then), Box::new(else_))
            }
            Expression::AssignOp(op, target, value) => {
                let target = self.optimize_expression(*target, value_map);
                let value = self.optimize_expression(*value, value_map);
                Expression::AssignOp(op, Box::new(target), Box::new(value))
            }
            Expression::PointerMember(expr, field) => {
                let expr = self.optimize_expression(*expr, value_map);
                Expression::PointerMember(Box::new(expr), field)
            }
            Expression::AddressOf(expr) => {
                let expr = self.optimize_expression(*expr, value_map);
                Expression::AddressOf(Box::new(expr))
            }
            Expression::SizeOf(ty) => Expression::SizeOf(ty),
            Expression::SizeOfExpr(expr) => {
                let expr = self.optimize_expression(*expr, value_map);
                Expression::SizeOfExpr(Box::new(expr))
            }
            Expression::AlignOf(ty) => Expression::AlignOf(ty),
            Expression::Comma(exprs) => {
                let exprs = exprs
                    .into_iter()
                    .map(|e| self.optimize_expression(e, value_map))
                    .collect();
                Expression::Comma(exprs)
            }
            Expression::Parenthesized(expr) => {
                let expr = self.optimize_expression(*expr, value_map);
                Expression::Parenthesized(Box::new(expr))
            }
            Expression::InitializerList(inits) => {
                // InitializerList doesn't need full recursion for now
                Expression::InitializerList(inits)
            }
            Expression::CompoundLiteral(ty, inits) => Expression::CompoundLiteral(ty, inits),
            // 这些不改变结构，递归优化子表达式
            Expression::Literal(_) => expr,
        }
    }

    /// 常量折叠二元运算
    fn fold_binary(&self, op: BinaryOp, left: Literal, right: Literal) -> Option<Literal> {
        match (left, right) {
            (Literal::Integer(l), Literal::Integer(r)) => {
                let result = match op {
                    BinaryOp::Add => Literal::Integer(l.wrapping_add(r)),
                    BinaryOp::Subtract => Literal::Integer(l.wrapping_sub(r)),
                    BinaryOp::Multiply => Literal::Integer(l.wrapping_mul(r)),
                    BinaryOp::Divide if r != 0 => Literal::Integer(l / r),
                    BinaryOp::Modulo if r != 0 => Literal::Integer(l % r),
                    BinaryOp::Equal => Literal::Bool(l == r),
                    BinaryOp::NotEqual => Literal::Bool(l != r),
                    BinaryOp::LessThan => Literal::Bool(l < r),
                    BinaryOp::LessThanEqual => Literal::Bool(l <= r),
                    BinaryOp::GreaterThan => Literal::Bool(l > r),
                    BinaryOp::GreaterThanEqual => Literal::Bool(l >= r),
                    BinaryOp::BitAnd => Literal::Integer(l & r),
                    BinaryOp::BitOr => Literal::Integer(l | r),
                    BinaryOp::BitXor => Literal::Integer(l ^ r),
                    BinaryOp::LeftShift if (0_i64..64).contains(&r) => Literal::Integer(l << r),
                    BinaryOp::RightShift if (0_i64..64).contains(&r) => Literal::Integer(l >> r),
                    BinaryOp::LogicalAnd => Literal::Bool(l != 0 && r != 0),
                    BinaryOp::LogicalOr => Literal::Bool(l != 0 || r != 0),
                    _ => return None,
                };
                Some(result)
            }
            (Literal::Double(l), Literal::Double(r)) => {
                let result = match op {
                    BinaryOp::Add => Literal::Double(l + r),
                    BinaryOp::Subtract => Literal::Double(l - r),
                    BinaryOp::Multiply => Literal::Double(l * r),
                    BinaryOp::Divide => Literal::Double(l / r),
                    BinaryOp::Equal => Literal::Bool(l == r),
                    BinaryOp::NotEqual => Literal::Bool(l != r),
                    BinaryOp::LessThan => Literal::Bool(l < r),
                    BinaryOp::LessThanEqual => Literal::Bool(l <= r),
                    BinaryOp::GreaterThan => Literal::Bool(l > r),
                    BinaryOp::GreaterThanEqual => Literal::Bool(l >= r),
                    _ => return None,
                };
                Some(result)
            }
            (Literal::Bool(l), Literal::Bool(r)) => {
                let result = match op {
                    BinaryOp::LogicalAnd => Literal::Bool(l && r),
                    BinaryOp::LogicalOr => Literal::Bool(l || r),
                    BinaryOp::Equal => Literal::Bool(l == r),
                    BinaryOp::NotEqual => Literal::Bool(l != r),
                    _ => return None,
                };
                Some(result)
            }
            _ => None,
        }
    }

    /// 常量折叠一元运算
    fn fold_unary(&self, op: UnaryOp, lit: Literal) -> Option<Literal> {
        match (op, lit) {
            (UnaryOp::Minus, Literal::Integer(i)) => Some(Literal::Integer(-i)),
            (UnaryOp::Minus, Literal::Double(f)) => Some(Literal::Double(-f)),
            (UnaryOp::Not, Literal::Bool(b)) => Some(Literal::Bool(!b)),
            (UnaryOp::BitNot, Literal::Integer(i)) => Some(Literal::Integer(!i)),
            _ => None,
        }
    }

    /// 常量折叠类型转换
    fn fold_cast(&self, ty: Type, lit: Literal) -> Option<Literal> {
        match (ty, lit) {
            (Type::Int, Literal::Integer(i)) => Some(Literal::Integer(i)),
            (Type::Int, Literal::Double(d)) => Some(Literal::Integer(d as i64)),
            (Type::Double, Literal::Integer(i)) => Some(Literal::Double(i as f64)),
            (Type::Bool, Literal::Integer(i)) => Some(Literal::Bool(i != 0)),
            (Type::Bool, Literal::Double(d)) => Some(Literal::Bool(d != 0.0)),
            _ => None,
        }
    }
}

impl Default for PeepholeOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

/// 对函数中所有基本块执行窥孔优化
pub fn peephole_optimize_function(block: &mut Block) {
    let opt = PeepholeOptimizer::new();
    opt.optimize_block(block);
}

/// 对整个程序所有函数执行窥孔优化
pub fn peephole_optimize_program(program: &mut crate::Program) {
    for decl in &mut program.declarations {
        if let crate::Declaration::Function(func) = decl {
            peephole_optimize_function(&mut func.body);
        }
    }
}

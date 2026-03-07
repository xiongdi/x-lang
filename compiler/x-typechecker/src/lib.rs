// 类型检查器库

pub mod errors;

use std::collections::HashMap;
use thiserror::Error;
use x_parser::ast::{Program, Declaration, Statement, Expression, Type, Literal, FunctionDecl, VariableDecl, Parameter, Block};

/// 类型检查器错误
#[derive(Error, Debug)]
pub enum TypeCheckError {
    #[error("类型不匹配: 期望 {expected}, 但实际是 {actual}")]
    TypeMismatch { expected: String, actual: String },

    #[error("未定义的变量: {0}")]
    UndefinedVariable(String),

    #[error("未定义的类型: {0}")]
    UndefinedType(String),

    #[error("函数参数数量不匹配: 期望 {expected}, 但实际是 {actual}")]
    ParameterCountMismatch { expected: usize, actual: usize },

    #[error("函数调用参数类型不匹配")]
    ParameterTypeMismatch,

    #[error("无法推断类型")]
    CannotInferType,

    #[error("类型参数数量不匹配")]
    TypeParameterCountMismatch,

    #[error("类型参数约束未满足")]
    TypeParameterConstraintViolated,

    #[error("递归类型定义")]
    RecursiveType,

    #[error("无效的类型注解")]
    InvalidTypeAnnotation,

    #[error("类型不兼容")]
    TypeIncompatible,
}

/// 类型环境
struct TypeEnv {
    variables: HashMap<String, Type>,
    functions: HashMap<String, Type>,
}

impl TypeEnv {
    fn new() -> Self {
        Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    fn add_variable(&mut self, name: &str, ty: Type) {
        self.variables.insert(name.to_string(), ty);
    }

    fn add_function(&mut self, name: &str, ty: Type) {
        self.functions.insert(name.to_string(), ty);
    }

    fn get_variable(&self, name: &str) -> Option<&Type> {
        self.variables.get(name)
    }

    fn get_function(&self, name: &str) -> Option<&Type> {
        self.functions.get(name)
    }
}

/// 类型检查器主函数
pub fn type_check(program: &Program) -> Result<(), TypeCheckError> {
    let mut env = TypeEnv::new();
    check_program(program, &mut env)
}

/// 检查程序
fn check_program(program: &Program, env: &mut TypeEnv) -> Result<(), TypeCheckError> {
    // 首先检查所有声明
    for decl in &program.declarations {
        check_declaration(decl, env)?;
    }

    // 然后检查所有语句
    for stmt in &program.statements {
        check_statement(stmt, env)?;
    }

    Ok(())
}

/// 检查声明
fn check_declaration(decl: &Declaration, env: &mut TypeEnv) -> Result<(), TypeCheckError> {
    match decl {
        Declaration::Variable(var_decl) => check_variable_decl(var_decl, env),
        Declaration::Function(func_decl) => check_function_decl(func_decl, env),
        Declaration::Class(_) => Ok(()), // 暂不实现
        Declaration::Trait(_) => Ok(()), // 暂不实现
        Declaration::TypeAlias(_) => Ok(()), // 暂不实现
        Declaration::Module(_) => Ok(()), // 暂不实现
        Declaration::Import(_) => Ok(()), // 暂不实现
        Declaration::Export(_) => Ok(()), // 暂不实现
    }
}

/// 检查变量声明
fn check_variable_decl(var_decl: &VariableDecl, env: &mut TypeEnv) -> Result<(), TypeCheckError> {
    // 检查初始化表达式的类型
    if let Some(initializer) = &var_decl.initializer {
        let init_type = infer_expression_type(initializer, env)?;
        
        // 如果有类型注解，检查类型匹配
        if let Some(type_annot) = &var_decl.type_annot {
            if !types_equal(&init_type, type_annot) {
                return Err(TypeCheckError::TypeMismatch {
                    expected: format!("{:?}", type_annot),
                    actual: format!("{:?}", init_type),
                });
            }
            env.add_variable(&var_decl.name, type_annot.clone());
        } else {
            // 没有类型注解，使用推断的类型
            env.add_variable(&var_decl.name, init_type);
        }
    } else if let Some(type_annot) = &var_decl.type_annot {
        // 只有类型注解，没有初始化表达式
        env.add_variable(&var_decl.name, type_annot.clone());
    } else {
        // 既没有类型注解也没有初始化表达式，无法推断类型
        return Err(TypeCheckError::CannotInferType);
    }

    Ok(())
}

/// 检查函数声明
fn check_function_decl(func_decl: &FunctionDecl, env: &mut TypeEnv) -> Result<(), TypeCheckError> {
    // 创建函数的类型
    let mut param_types = Vec::new();
    for param in &func_decl.parameters {
        if let Some(type_annot) = &param.type_annot {
            param_types.push(Box::new(type_annot.clone()));
        } else {
            // 参数必须有类型注解
            return Err(TypeCheckError::CannotInferType);
        }
    }

    let return_type = if let Some(return_type) = &func_decl.return_type {
        Box::new(return_type.clone())
    } else {
        Box::new(Type::Unit)
    };

    let func_type = Type::Function(param_types, return_type);
    env.add_function(&func_decl.name, func_type);

    // 检查函数体
    check_block(&func_decl.body, env)
}

/// 检查语句
fn check_statement(stmt: &Statement, env: &mut TypeEnv) -> Result<(), TypeCheckError> {
    match stmt {
        Statement::Expression(expr) => {
            infer_expression_type(expr, env)?;
            Ok(())
        }
        Statement::Variable(var_decl) => check_variable_decl(var_decl, env),
        Statement::Return(expr_opt) => {
            if let Some(expr) = expr_opt {
                infer_expression_type(expr, env)?;
            }
            Ok(())
        }
        Statement::If(if_stmt) => {
            // 检查条件表达式类型为布尔
            let cond_type = infer_expression_type(&if_stmt.condition, env)?;
            if !types_equal(&cond_type, &Type::Bool) {
                return Err(TypeCheckError::TypeMismatch {
                    expected: format!("{:?}", Type::Bool),
                    actual: format!("{:?}", cond_type),
                });
            }
            
            // 检查then块
            check_block(&if_stmt.then_block, env)?;
            
            // 检查else块
            if let Some(else_block) = &if_stmt.else_block {
                check_block(else_block, env)?;
            }
            
            Ok(())
        }
        Statement::For(_) => Ok(()), // 暂不实现
        Statement::While(while_stmt) => {
            // 检查条件表达式类型为布尔
            let cond_type = infer_expression_type(&while_stmt.condition, env)?;
            if !types_equal(&cond_type, &Type::Bool) {
                return Err(TypeCheckError::TypeMismatch {
                    expected: format!("{:?}", Type::Bool),
                    actual: format!("{:?}", cond_type),
                });
            }
            
            // 检查循环体
            check_block(&while_stmt.body, env)
        }
        Statement::Match(_) => Ok(()), // 暂不实现
        Statement::Try(_) => Ok(()), // 暂不实现
    }
}

/// 检查块语句
fn check_block(block: &Block, env: &mut TypeEnv) -> Result<(), TypeCheckError> {
    for stmt in &block.statements {
        check_statement(stmt, env)?;
    }
    Ok(())
}

/// 推断表达式类型
fn infer_expression_type(expr: &Expression, env: &TypeEnv) -> Result<Type, TypeCheckError> {
    match expr {
        Expression::Literal(lit) => infer_literal_type(lit),
        Expression::Variable(name) => {
            if let Some(ty) = env.get_variable(name) {
                Ok(ty.clone())
            } else if let Some(ty) = env.get_function(name) {
                Ok(ty.clone())
            } else {
                Err(TypeCheckError::UndefinedVariable(name.to_string()))
            }
        }
        Expression::Member(_, _) => Ok(Type::Unit), // 暂不实现
        Expression::Call(callee, args) => {
            // 推断被调用表达式的类型
            let callee_type = infer_expression_type(callee, env)?;
            
            // 检查是否为函数类型
            if let Type::Function(param_types, return_type) = callee_type {
                // 检查参数数量
                if param_types.len() != args.len() {
                    return Err(TypeCheckError::ParameterCountMismatch {
                        expected: param_types.len(),
                        actual: args.len(),
                    });
                }
                
                // 检查参数类型
                for (i, (param_type, arg)) in param_types.iter().zip(args).enumerate() {
                    let arg_type = infer_expression_type(arg, env)?;
                    if !types_equal(&arg_type, &param_type) {
                        return Err(TypeCheckError::ParameterTypeMismatch);
                    }
                }
                
                Ok(*return_type)
            } else {
                Err(TypeCheckError::TypeMismatch {
                    expected: "Function".to_string(),
                    actual: format!("{:?}", callee_type),
                })
            }
        }
        Expression::Binary(op, left, right) => {
            let left_type = infer_expression_type(left, env)?;
            let right_type = infer_expression_type(right, env)?;
            
            // 检查左右操作数类型是否匹配
            if !types_equal(&left_type, &right_type) {
                return Err(TypeCheckError::TypeMismatch {
                    expected: format!("{:?}", left_type),
                    actual: format!("{:?}", right_type),
                });
            }
            
            // 根据操作符返回相应的类型
            match op {
                // 算术运算返回数值类型
                x_parser::ast::BinaryOp::Add | x_parser::ast::BinaryOp::Sub | 
                x_parser::ast::BinaryOp::Mul | x_parser::ast::BinaryOp::Div | 
                x_parser::ast::BinaryOp::Mod | x_parser::ast::BinaryOp::Pow => {
                    if types_equal(&left_type, &Type::Int) || types_equal(&left_type, &Type::Float) {
                        Ok(left_type)
                    } else {
                        Err(TypeCheckError::TypeMismatch {
                            expected: "Int or Float".to_string(),
                            actual: format!("{:?}", left_type),
                        })
                    }
                }
                // 逻辑运算返回布尔类型
                x_parser::ast::BinaryOp::And | x_parser::ast::BinaryOp::Or => {
                    if types_equal(&left_type, &Type::Bool) {
                        Ok(Type::Bool)
                    } else {
                        Err(TypeCheckError::TypeMismatch {
                            expected: format!("{:?}", Type::Bool),
                            actual: format!("{:?}", left_type),
                        })
                    }
                }
                // 比较运算返回布尔类型
                x_parser::ast::BinaryOp::Equal | x_parser::ast::BinaryOp::NotEqual | 
                x_parser::ast::BinaryOp::Less | x_parser::ast::BinaryOp::LessEqual | 
                x_parser::ast::BinaryOp::Greater | x_parser::ast::BinaryOp::GreaterEqual => {
                    Ok(Type::Bool)
                }
                _ => Ok(Type::Unit), // 其他操作暂不实现
            }
        }
        Expression::Unary(op, expr) => {
            let expr_type = infer_expression_type(expr, env)?;
            match op {
                x_parser::ast::UnaryOp::Negate => {
                    if types_equal(&expr_type, &Type::Int) || types_equal(&expr_type, &Type::Float) {
                        Ok(expr_type)
                    } else {
                        Err(TypeCheckError::TypeMismatch {
                            expected: "Int or Float".to_string(),
                            actual: format!("{:?}", expr_type),
                        })
                    }
                }
                x_parser::ast::UnaryOp::Not => {
                    if types_equal(&expr_type, &Type::Bool) {
                        Ok(Type::Bool)
                    } else {
                        Err(TypeCheckError::TypeMismatch {
                            expected: format!("{:?}", Type::Bool),
                            actual: format!("{:?}", expr_type),
                        })
                    }
                }
                _ => Ok(Type::Unit), // 其他操作暂不实现
            }
        }
        Expression::Assign(lhs, rhs) => {
            // 推断右侧表达式类型
            let rhs_type = infer_expression_type(rhs, env)?;
            
            // 推断左侧表达式类型
            let lhs_type = infer_expression_type(lhs, env)?;
            
            // 检查类型匹配
            if !types_equal(&lhs_type, &rhs_type) {
                return Err(TypeCheckError::TypeMismatch {
                    expected: format!("{:?}", lhs_type),
                    actual: format!("{:?}", rhs_type),
                });
            }
            
            Ok(rhs_type)
        }
        Expression::If(cond, then_expr, else_expr) => {
            // 检查条件表达式类型为布尔
            let cond_type = infer_expression_type(cond, env)?;
            if !types_equal(&cond_type, &Type::Bool) {
                return Err(TypeCheckError::TypeMismatch {
                    expected: format!("{:?}", Type::Bool),
                    actual: format!("{:?}", cond_type),
                });
            }
            
            // 推断then和else表达式类型
            let then_type = infer_expression_type(then_expr, env)?;
            let else_type = infer_expression_type(else_expr, env)?;
            
            // 检查then和else表达式类型是否匹配
            if !types_equal(&then_type, &else_type) {
                return Err(TypeCheckError::TypeMismatch {
                    expected: format!("{:?}", then_type),
                    actual: format!("{:?}", else_type),
                });
            }
            
            Ok(then_type)
        }
        _ => Ok(Type::Unit), // 其他表达式暂不实现
    }
}

/// 推断字面量类型
fn infer_literal_type(lit: &Literal) -> Result<Type, TypeCheckError> {
    match lit {
        Literal::Integer(_) => Ok(Type::Int),
        Literal::Float(_) => Ok(Type::Float),
        Literal::Boolean(_) => Ok(Type::Bool),
        Literal::String(_) => Ok(Type::String),
        Literal::Char(_) => Ok(Type::Char),
        Literal::Null => Ok(Type::Unit),
        Literal::None => Ok(Type::Option(Box::new(Type::Unit))),
        Literal::Unit => Ok(Type::Unit),
    }
}

/// 检查两个类型是否相等
fn types_equal(ty1: &Type, ty2: &Type) -> bool {
    match (ty1, ty2) {
        (Type::Int, Type::Int) => true,
        (Type::Float, Type::Float) => true,
        (Type::Bool, Type::Bool) => true,
        (Type::String, Type::String) => true,
        (Type::Char, Type::Char) => true,
        (Type::Unit, Type::Unit) => true,
        (Type::Never, Type::Never) => true,
        (Type::Array(a1), Type::Array(a2)) => types_equal(a1, a2),
        (Type::Dictionary(k1, v1), Type::Dictionary(k2, v2)) => {
            types_equal(k1, k2) && types_equal(v1, v2)
        }
        (Type::Function(p1, r1), Type::Function(p2, r2)) => {
            if p1.len() != p2.len() {
                return false;
            }
            for (t1, t2) in p1.iter().zip(p2) {
                if !types_equal(t1, t2) {
                    return false;
                }
            }
            types_equal(r1, r2)
        }
        _ => false,
    }
}
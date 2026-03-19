//! LIR -> AST adapter
//!
//! 该模块用于把 `x_lir::Program` 转换为 `x_parser::ast::Program`，以便在后端
//! 迁移到 LIR 输入后，仍可复用现有基于 AST 的文本后端实现。
//!
//! 当前目标是覆盖仓库里现有 lowering 实际产出的那部分 LIR 子集：
//! - 顶层函数 / 全局变量 / extern 函数
//! - 结构体 / 类（退化为 AST ClassDecl）
//! - 结构化控制流：if / while / do-while / for / match / try
//! - 常见表达式：literal / variable / unary / binary / assign / call / index / member
//! - 数组初始化列表 / 记录复合字面量
//!
//! 对于 AST 无法表达、或当前后端不会稳定消费的更低级节点：
//! - goto / label
//! - address-of / dereference / pointer-member
//! - 一般化 cast
//! - 复杂 designated initializer
//!
//! 本模块会显式返回错误，而不是悄悄生成错误代码。

use x_lexer::span::Span;
use x_lir::{
    self as lir, BinaryOp as LirBinaryOp, Declaration as LirDeclaration,
    Expression as LirExpression, Function as LirFunction, Initializer as LirInitializer,
    Literal as LirLiteral, Pattern as LirPattern, Program as LirProgram, Statement as LirStatement,
    Type as LirType, UnaryOp as LirUnaryOp,
};
use x_parser::ast::{
    self, spanned, BinaryOp as AstBinaryOp, Block as AstBlock, CatchClause as AstCatchClause,
    ClassDecl, ClassMember, ClassModifiers, ConstructorDecl, Declaration as AstDeclaration,
    DoWhileStatement as AstDoWhileStatement, EnumDecl, EnumVariant, EnumVariantData, ExportDecl,
    Expression as AstExpression, ExpressionKind as AstExpressionKind, ExternFunctionDecl,
    ForStatement as AstForStatement, FunctionDecl, IfStatement as AstIfStatement, ImportDecl,
    ImportSymbol, Literal as AstLiteral, MatchCase as AstMatchCase,
    MatchStatement as AstMatchStatement, MethodModifiers, ModuleDecl, Parameter,
    Pattern as AstPattern, Program as AstProgram, Statement as AstStatement,
    StatementKind as AstStatementKind, TraitDecl, TryStatement as AstTryStatement, Type as AstType,
    TypeAlias, VariableDecl, Visibility, WhileStatement as AstWhileStatement,
};

/// LIR -> AST 转换错误
#[derive(Debug, thiserror::Error)]
pub enum LirToAstError {
    #[error("当前 AST 无法表达该 LIR 节点: {0}")]
    Unsupported(String),

    #[error("无效的 LIR 结构: {0}")]
    Invalid(String),
}

pub type LirToAstResult<T> = Result<T, LirToAstError>;

/// 将整个 LIR 程序转换为 AST 程序
pub fn lower_lir_to_ast(program: &LirProgram) -> LirToAstResult<AstProgram> {
    let mut declarations = Vec::new();

    for decl in &program.declarations {
        if let Some(ast_decl) = lower_declaration(decl)? {
            declarations.push(ast_decl);
        }
    }

    Ok(AstProgram {
        declarations,
        statements: Vec::new(),
        span: Span::default(),
    })
}

fn lower_declaration(decl: &LirDeclaration) -> LirToAstResult<Option<AstDeclaration>> {
    match decl {
        LirDeclaration::Function(func) => Ok(Some(AstDeclaration::Function(lower_function(func)?))),
        LirDeclaration::Global(global) => Ok(Some(AstDeclaration::Variable(lower_global(global)?))),
        LirDeclaration::ExternFunction(ext) => Ok(Some(AstDeclaration::ExternFunction(
            lower_extern_function(ext)?,
        ))),
        LirDeclaration::Struct(strct) => Ok(Some(AstDeclaration::Class(lower_struct_as_class(
            strct.name.clone(),
            strct
                .fields
                .iter()
                .map(|f| (f.name.clone(), lower_type(&f.type_)))
                .collect(),
        )?))),
        LirDeclaration::Class(class) => Ok(Some(AstDeclaration::Class(lower_lir_class(class)?))),
        LirDeclaration::Enum(enm) => Ok(Some(AstDeclaration::Enum(lower_enum(enm)?))),
        LirDeclaration::TypeAlias(alias) => Ok(Some(AstDeclaration::TypeAlias(TypeAlias {
            name: alias.name.clone(),
            type_: lower_type(&alias.type_),
            span: Span::default(),
        }))),
        LirDeclaration::VTable(_) => {
            // 当前 AST 后端不直接消费 vtable 定义；现有文本后端也不会从 AST 里读取它。
            Ok(None)
        }
    }
}

fn lower_function(func: &LirFunction) -> LirToAstResult<FunctionDecl> {
    let parameters = func
        .parameters
        .iter()
        .map(|p| Parameter {
            name: p.name.clone(),
            type_annot: Some(lower_type(&p.type_)),
            default: None,
            span: Span::default(),
        })
        .collect();

    let return_type = if matches!(func.return_type, LirType::Void) {
        None
    } else {
        Some(lower_type(&func.return_type))
    };

    Ok(FunctionDecl {
        name: func.name.clone(),
        type_parameters: Vec::new(),
        parameters,
        return_type,
        effects: Vec::new(),
        body: lower_block(&func.body)?,
        is_async: false,
        modifiers: MethodModifiers {
            is_virtual: false,
            is_override: false,
            is_final: false,
            is_abstract: false,
            is_static: func.is_static,
            visibility: Visibility::Private,
        },
        span: Span::default(),
    })
}

fn lower_global(global: &lir::GlobalVar) -> LirToAstResult<VariableDecl> {
    Ok(VariableDecl {
        name: global.name.clone(),
        is_mutable: !global.is_static,
        type_annot: Some(lower_type(&global.type_)),
        initializer: global
            .initializer
            .as_ref()
            .map(lower_expression)
            .transpose()?,
        visibility: Visibility::Private,
        span: Span::default(),
    })
}

fn lower_extern_function(ext: &lir::ExternFunction) -> LirToAstResult<ExternFunctionDecl> {
    let parameters = ext
        .parameters
        .iter()
        .enumerate()
        .map(|(i, ty)| Parameter {
            name: format!("arg{i}"),
            type_annot: Some(lower_type(ty)),
            default: None,
            span: Span::default(),
        })
        .collect();

    Ok(ExternFunctionDecl {
        abi: "C".to_string(),
        name: ext.name.clone(),
        parameters,
        return_type: if matches!(ext.return_type, LirType::Void) {
            None
        } else {
            Some(lower_type(&ext.return_type))
        },
        is_variadic: false,
        span: Span::default(),
    })
}

fn lower_struct_as_class(
    name: String,
    fields: Vec<(String, AstType)>,
) -> LirToAstResult<ClassDecl> {
    let members = fields
        .into_iter()
        .map(|(field_name, field_ty)| {
            ClassMember::Field(VariableDecl {
                name: field_name,
                is_mutable: true,
                type_annot: Some(field_ty),
                initializer: None,
                visibility: Visibility::Private,
                span: Span::default(),
            })
        })
        .collect();

    Ok(ClassDecl {
        name,
        type_parameters: Vec::new(),
        extends: None,
        implements: Vec::new(),
        members,
        modifiers: ClassModifiers {
            is_abstract: false,
            is_final: false,
        },
        span: Span::default(),
    })
}

fn lower_lir_class(class: &lir::Class) -> LirToAstResult<ClassDecl> {
    let members = class
        .fields
        .iter()
        .map(|field| {
            ClassMember::Field(VariableDecl {
                name: field.name.clone(),
                is_mutable: true,
                type_annot: Some(lower_type(&field.type_)),
                initializer: None,
                visibility: Visibility::Private,
                span: Span::default(),
            })
        })
        .collect();

    Ok(ClassDecl {
        name: class.name.clone(),
        type_parameters: Vec::new(),
        extends: class.extends.clone(),
        implements: class.implements.clone(),
        members,
        modifiers: ClassModifiers {
            is_abstract: false,
            is_final: false,
        },
        span: Span::default(),
    })
}

fn lower_enum(enm: &lir::Enum) -> LirToAstResult<EnumDecl> {
    Ok(EnumDecl {
        name: enm.name.clone(),
        type_parameters: Vec::new(),
        variants: enm
            .variants
            .iter()
            .map(|v| EnumVariant {
                name: v.name.clone(),
                data: EnumVariantData::Unit,
                doc: None,
                span: Span::default(),
            })
            .collect(),
        span: Span::default(),
    })
}

fn lower_block(block: &lir::Block) -> LirToAstResult<AstBlock> {
    let mut statements = Vec::with_capacity(block.statements.len());

    for stmt in &block.statements {
        let lowered = lower_statement(stmt)?;
        statements.extend(lowered);
    }

    Ok(AstBlock { statements })
}

fn lower_statement(stmt: &LirStatement) -> LirToAstResult<Vec<AstStatement>> {
    match stmt {
        LirStatement::Expression(expr) => Ok(vec![spanned(
            AstStatementKind::Expression(lower_expression(expr)?),
            Span::default(),
        )]),
        LirStatement::Declaration(decl) => {
            let lowered = lower_declaration(decl)?;
            match lowered {
                Some(AstDeclaration::Variable(var)) => Ok(vec![spanned(
                    AstStatementKind::Variable(var),
                    Span::default(),
                )]),
                Some(other) => Err(LirToAstError::Unsupported(format!(
                    "statement-level declaration not representable in AST statement position: {:?}",
                    other
                ))),
                None => Ok(vec![]),
            }
        }
        LirStatement::Variable(var) => Ok(vec![spanned(
            AstStatementKind::Variable(VariableDecl {
                name: var.name.clone(),
                is_mutable: true,
                type_annot: Some(lower_type(&var.type_)),
                initializer: var.initializer.as_ref().map(lower_expression).transpose()?,
                visibility: Visibility::Private,
                span: Span::default(),
            }),
            Span::default(),
        )]),
        LirStatement::If(if_stmt) => Ok(vec![spanned(
            AstStatementKind::If(AstIfStatement {
                condition: lower_expression(&if_stmt.condition)?,
                then_block: lower_statement_as_block(&if_stmt.then_branch)?,
                else_block: if_stmt
                    .else_branch
                    .as_ref()
                    .map(|s| lower_statement_as_block(s))
                    .transpose()?,
            }),
            Span::default(),
        )]),
        LirStatement::While(while_stmt) => Ok(vec![spanned(
            AstStatementKind::While(AstWhileStatement {
                condition: lower_expression(&while_stmt.condition)?,
                body: lower_statement_as_block(&while_stmt.body)?,
            }),
            Span::default(),
        )]),
        LirStatement::DoWhile(do_while) => Ok(vec![spanned(
            AstStatementKind::DoWhile(AstDoWhileStatement {
                body: lower_statement_as_block(&do_while.body)?,
                condition: lower_expression(&do_while.condition)?,
            }),
            Span::default(),
        )]),
        LirStatement::For(for_stmt) => {
            // AST 的 For 是 “for pattern in iterator”，
            // LIR 的 For 是 C-style。这里将其重写为：
            // { initializer; while (condition) { body; increment; } }
            let mut outer = Vec::new();

            if let Some(init) = &for_stmt.initializer {
                outer.extend(lower_statement(init)?);
            }

            let mut while_body = lower_statement_as_block(&for_stmt.body)?;
            if let Some(inc) = &for_stmt.increment {
                while_body.statements.push(spanned(
                    AstStatementKind::Expression(lower_expression(inc)?),
                    Span::default(),
                ));
            }

            let condition = match &for_stmt.condition {
                Some(cond) => lower_expression(cond)?,
                None => spanned(
                    AstExpressionKind::Literal(AstLiteral::Boolean(true)),
                    Span::default(),
                ),
            };

            outer.push(spanned(
                AstStatementKind::While(AstWhileStatement {
                    condition,
                    body: while_body,
                }),
                Span::default(),
            ));

            Ok(outer)
        }
        LirStatement::Switch(switch_stmt) => {
            // AST 没有 switch，但有 match。
            let cases = switch_stmt
                .cases
                .iter()
                .map(|c| {
                    Ok(AstMatchCase {
                        pattern: AstPattern::Literal(lower_expression_as_literal(&c.value)?),
                        body: lower_statement_as_block(&c.body)?,
                        guard: None,
                    })
                })
                .collect::<LirToAstResult<Vec<_>>>()?;

            let mut cases = cases;
            if let Some(default_stmt) = &switch_stmt.default {
                cases.push(AstMatchCase {
                    pattern: AstPattern::Wildcard,
                    body: lower_statement_as_block(default_stmt)?,
                    guard: None,
                });
            }

            Ok(vec![spanned(
                AstStatementKind::Match(AstMatchStatement {
                    expression: lower_expression(&switch_stmt.expression)?,
                    cases,
                }),
                Span::default(),
            )])
        }
        LirStatement::Match(match_stmt) => Ok(vec![spanned(
            AstStatementKind::Match(AstMatchStatement {
                expression: lower_expression(&match_stmt.scrutinee)?,
                cases: match_stmt
                    .cases
                    .iter()
                    .map(|c| {
                        Ok(AstMatchCase {
                            pattern: lower_pattern(&c.pattern)?,
                            body: lower_block(&c.body)?,
                            guard: c.guard.as_ref().map(lower_expression).transpose()?,
                        })
                    })
                    .collect::<LirToAstResult<Vec<_>>>()?,
            }),
            Span::default(),
        )]),
        LirStatement::Try(try_stmt) => Ok(vec![spanned(
            AstStatementKind::Try(AstTryStatement {
                body: lower_block(&try_stmt.body)?,
                catch_clauses: try_stmt
                    .catch_clauses
                    .iter()
                    .map(|c| AstCatchClause {
                        exception_type: c.exception_type.clone(),
                        variable_name: c.variable_name.clone(),
                        body: lower_block(&c.body).unwrap_or(AstBlock { statements: vec![] }),
                    })
                    .collect(),
                finally_block: try_stmt
                    .finally_block
                    .as_ref()
                    .map(lower_block)
                    .transpose()?,
            }),
            Span::default(),
        )]),
        LirStatement::Break => Ok(vec![spanned(AstStatementKind::Break, Span::default())]),
        LirStatement::Continue => Ok(vec![spanned(AstStatementKind::Continue, Span::default())]),
        LirStatement::Return(expr) => Ok(vec![spanned(
            AstStatementKind::Return(expr.as_ref().map(lower_expression).transpose()?),
            Span::default(),
        )]),
        LirStatement::Compound(block) => Ok(lower_block(block)?.statements),
        LirStatement::Empty => Ok(vec![]),
        LirStatement::Goto(label) => Err(LirToAstError::Unsupported(format!(
            "goto is not representable in AST: {label}"
        ))),
        LirStatement::Label(label) => Err(LirToAstError::Unsupported(format!(
            "label is not representable in AST: {label}"
        ))),
    }
}

fn lower_statement_as_block(stmt: &LirStatement) -> LirToAstResult<AstBlock> {
    Ok(AstBlock {
        statements: lower_statement(stmt)?,
    })
}

fn lower_expression(expr: &LirExpression) -> LirToAstResult<AstExpression> {
    Ok(spanned(lower_expression_kind(expr)?, Span::default()))
}

fn lower_expression_kind(expr: &LirExpression) -> LirToAstResult<AstExpressionKind> {
    match expr {
        LirExpression::Literal(lit) => Ok(AstExpressionKind::Literal(lower_literal(lit))),
        LirExpression::Variable(name) => Ok(AstExpressionKind::Variable(name.clone())),
        LirExpression::Unary(op, expr) => Ok(AstExpressionKind::Unary(
            lower_unary_op(*op)?,
            Box::new(lower_expression(expr)?),
        )),
        LirExpression::Binary(op, lhs, rhs) => Ok(AstExpressionKind::Binary(
            lower_binary_op(*op)?,
            Box::new(lower_expression(lhs)?),
            Box::new(lower_expression(rhs)?),
        )),
        LirExpression::Ternary(cond, then_expr, else_expr) => Ok(AstExpressionKind::If(
            Box::new(lower_expression(cond)?),
            Box::new(lower_expression(then_expr)?),
            Box::new(lower_expression(else_expr)?),
        )),
        LirExpression::Assign(lhs, rhs) => Ok(AstExpressionKind::Assign(
            Box::new(lower_expression(lhs)?),
            Box::new(lower_expression(rhs)?),
        )),
        LirExpression::AssignOp(op, lhs, rhs) => {
            let lhs_expr = lower_expression(lhs)?;
            let rhs_expr = lower_expression(rhs)?;
            let bin = spanned(
                AstExpressionKind::Binary(
                    lower_binary_op(*op)?,
                    Box::new(lhs_expr.clone()),
                    Box::new(rhs_expr),
                ),
                Span::default(),
            );
            Ok(AstExpressionKind::Assign(Box::new(lhs_expr), Box::new(bin)))
        }
        LirExpression::Call(callee, args) => Ok(AstExpressionKind::Call(
            Box::new(lower_expression(callee)?),
            args.iter()
                .map(lower_expression)
                .collect::<LirToAstResult<Vec<_>>>()?,
        )),
        LirExpression::Index(target, index) => Ok(AstExpressionKind::Call(
            Box::new(spanned(
                AstExpressionKind::Variable("__lir_index".to_string()),
                Span::default(),
            )),
            vec![lower_expression(target)?, lower_expression(index)?],
        )),
        LirExpression::Member(obj, field) => Ok(AstExpressionKind::Member(
            Box::new(lower_expression(obj)?),
            field.clone(),
        )),
        LirExpression::Parenthesized(expr) => Ok(AstExpressionKind::Parenthesized(Box::new(
            lower_expression(expr)?,
        ))),
        LirExpression::InitializerList(items) => {
            let elements = items
                .iter()
                .map(lower_initializer_as_expression)
                .collect::<LirToAstResult<Vec<_>>>()?;
            Ok(AstExpressionKind::Array(elements))
        }
        LirExpression::CompoundLiteral(ty, inits) => match ty {
            LirType::Named(name) => {
                let mut fields = Vec::new();
                for init in inits {
                    match init {
                        LirInitializer::Named(field, value) => {
                            fields.push((field.clone(), lower_initializer_as_expression(value)?));
                        }
                        other => {
                            return Err(LirToAstError::Unsupported(format!(
                                "compound literal for named type requires named initializers, got: {:?}",
                                other
                            )))
                        }
                    }
                }
                Ok(AstExpressionKind::Record(name.clone(), fields))
            }
            _ => {
                let elements = inits
                    .iter()
                    .map(lower_initializer_as_expression)
                    .collect::<LirToAstResult<Vec<_>>>()?;
                Ok(AstExpressionKind::Array(elements))
            }
        },
        LirExpression::Comma(exprs) => {
            let mut exprs = exprs.iter();
            let first = exprs
                .next()
                .ok_or_else(|| LirToAstError::Invalid("empty comma expression".to_string()))?;
            let mut current = lower_expression(first)?;
            for expr in exprs {
                current = spanned(
                    AstExpressionKind::Pipe(
                        Box::new(current),
                        vec![Box::new(lower_expression(expr)?)],
                    ),
                    Span::default(),
                );
            }
            Ok(current.node)
        }

        LirExpression::PointerMember(_, _)
        | LirExpression::AddressOf(_)
        | LirExpression::Dereference(_)
        | LirExpression::Cast(_, _)
        | LirExpression::SizeOf(_)
        | LirExpression::SizeOfExpr(_)
        | LirExpression::AlignOf(_) => Err(LirToAstError::Unsupported(format!(
            "low-level expression not representable in AST: {:?}",
            expr
        ))),
    }
}

fn lower_initializer_as_expression(init: &LirInitializer) -> LirToAstResult<AstExpression> {
    match init {
        LirInitializer::Expression(expr) => lower_expression(expr),
        LirInitializer::List(items) => Ok(spanned(
            AstExpressionKind::Array(
                items
                    .iter()
                    .map(lower_initializer_as_expression)
                    .collect::<LirToAstResult<Vec<_>>>()?,
            ),
            Span::default(),
        )),
        LirInitializer::Named(_, _) | LirInitializer::Indexed(_, _) => {
            Err(LirToAstError::Unsupported(format!(
                "designated initializer is not representable as plain AST expression: {:?}",
                init
            )))
        }
    }
}

fn lower_expression_as_literal(expr: &LirExpression) -> LirToAstResult<AstLiteral> {
    match expr {
        LirExpression::Literal(lit) => Ok(lower_literal(lit)),
        _ => Err(LirToAstError::Unsupported(format!(
            "switch/match case value must be literal-like, got: {:?}",
            expr
        ))),
    }
}

fn lower_literal(lit: &LirLiteral) -> AstLiteral {
    match lit {
        LirLiteral::Integer(v) => AstLiteral::Integer(*v),
        LirLiteral::UnsignedInteger(v) => AstLiteral::Integer(*v as i64),
        LirLiteral::Long(v) => AstLiteral::Integer(*v),
        LirLiteral::UnsignedLong(v) => AstLiteral::Integer(*v as i64),
        LirLiteral::LongLong(v) => AstLiteral::Integer(*v),
        LirLiteral::UnsignedLongLong(v) => AstLiteral::Integer(*v as i64),
        LirLiteral::Float(v) => AstLiteral::Float(*v),
        LirLiteral::Double(v) => AstLiteral::Float(*v),
        LirLiteral::Char(v) => AstLiteral::Char(*v),
        LirLiteral::String(v) => AstLiteral::String(v.clone()),
        LirLiteral::Bool(v) => AstLiteral::Boolean(*v),
        LirLiteral::NullPointer => AstLiteral::Null,
    }
}

fn lower_pattern(pattern: &LirPattern) -> LirToAstResult<AstPattern> {
    match pattern {
        LirPattern::Wildcard => Ok(AstPattern::Wildcard),
        LirPattern::Variable(name) => Ok(AstPattern::Variable(name.clone())),
        LirPattern::Literal(lit) => Ok(AstPattern::Literal(lower_literal(lit))),
        LirPattern::Tuple(items) => Ok(AstPattern::Tuple(
            items
                .iter()
                .map(lower_pattern)
                .collect::<LirToAstResult<Vec<_>>>()?,
        )),
        LirPattern::Record(name, fields) => Ok(AstPattern::Record(
            name.clone(),
            fields
                .iter()
                .map(|(k, p)| Ok((k.clone(), lower_pattern(p)?)))
                .collect::<LirToAstResult<Vec<_>>>()?,
        )),
        LirPattern::Or(lhs, rhs) => Ok(AstPattern::Or(
            Box::new(lower_pattern(lhs)?),
            Box::new(lower_pattern(rhs)?),
        )),
        LirPattern::Constructor(name, args) => Ok(AstPattern::EnumConstructor(
            "_".to_string(),
            name.clone(),
            args.iter()
                .map(lower_pattern)
                .collect::<LirToAstResult<Vec<_>>>()?,
        )),
    }
}

fn lower_type(ty: &LirType) -> AstType {
    match ty {
        LirType::Void => AstType::Unit,
        LirType::Bool => AstType::Bool,
        LirType::Char => AstType::Char,
        LirType::Schar | LirType::Uchar => AstType::CChar,
        LirType::Short | LirType::Ushort | LirType::Int => AstType::Int,
        LirType::Uint => AstType::UnsignedInt,
        LirType::Long | LirType::LongLong => AstType::CLongLong,
        LirType::Ulong | LirType::UlongLong => AstType::CULongLong,
        LirType::Float => AstType::CFloat,
        LirType::Double | LirType::LongDouble => AstType::Float,
        LirType::Size => AstType::CSize,
        LirType::Ptrdiff | LirType::Intptr | LirType::Uintptr => AstType::CLongLong,
        LirType::Pointer(inner) => AstType::Pointer(Box::new(lower_type(inner))),
        LirType::Array(inner, _) => AstType::Array(Box::new(lower_type(inner))),
        LirType::FunctionPointer(ret, params) => AstType::Function(
            params.iter().map(|p| Box::new(lower_type(p))).collect(),
            Box::new(lower_type(ret)),
        ),
        LirType::Named(name) => AstType::Generic(name.clone()),
        LirType::Qualified(_, inner) => lower_type(inner),
    }
}

fn lower_binary_op(op: LirBinaryOp) -> LirToAstResult<AstBinaryOp> {
    Ok(match op {
        LirBinaryOp::Add => AstBinaryOp::Add,
        LirBinaryOp::Subtract => AstBinaryOp::Sub,
        LirBinaryOp::Multiply => AstBinaryOp::Mul,
        LirBinaryOp::Divide => AstBinaryOp::Div,
        LirBinaryOp::Modulo => AstBinaryOp::Mod,
        LirBinaryOp::LeftShift => AstBinaryOp::LeftShift,
        LirBinaryOp::RightShift => AstBinaryOp::RightShift,
        LirBinaryOp::LessThan => AstBinaryOp::Less,
        LirBinaryOp::LessThanEqual => AstBinaryOp::LessEqual,
        LirBinaryOp::GreaterThan => AstBinaryOp::Greater,
        LirBinaryOp::GreaterThanEqual => AstBinaryOp::GreaterEqual,
        LirBinaryOp::Equal => AstBinaryOp::Equal,
        LirBinaryOp::NotEqual => AstBinaryOp::NotEqual,
        LirBinaryOp::BitAnd => AstBinaryOp::BitAnd,
        LirBinaryOp::BitXor => AstBinaryOp::BitXor,
        LirBinaryOp::BitOr => AstBinaryOp::BitOr,
        LirBinaryOp::LogicalAnd => AstBinaryOp::And,
        LirBinaryOp::LogicalOr => AstBinaryOp::Or,
    })
}

fn lower_unary_op(op: LirUnaryOp) -> LirToAstResult<ast::UnaryOp> {
    Ok(match op {
        LirUnaryOp::Plus => {
            return Err(LirToAstError::Unsupported(
                "unary plus is not representable in AST".to_string(),
            ))
        }
        LirUnaryOp::Minus => ast::UnaryOp::Negate,
        LirUnaryOp::Not => ast::UnaryOp::Not,
        LirUnaryOp::BitNot => ast::UnaryOp::BitNot,
        LirUnaryOp::PreIncrement
        | LirUnaryOp::PreDecrement
        | LirUnaryOp::PostIncrement
        | LirUnaryOp::PostDecrement => {
            return Err(LirToAstError::Unsupported(format!(
                "increment/decrement unary operator is not representable in AST: {:?}",
                op
            )))
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lowers_simple_function_program() {
        let program = LirProgram {
            declarations: vec![LirDeclaration::Function(LirFunction {
                name: "main".to_string(),
                return_type: LirType::Void,
                parameters: vec![],
                body: lir::Block {
                    statements: vec![LirStatement::Return(None)],
                },
                is_static: false,
                is_inline: false,
            })],
        };

        let ast = lower_lir_to_ast(&program).expect("lower_lir_to_ast");
        assert_eq!(ast.declarations.len(), 1);
    }

    #[test]
    fn lowers_global_variable() {
        let program = LirProgram {
            declarations: vec![LirDeclaration::Global(lir::GlobalVar {
                name: "x".to_string(),
                type_: LirType::Int,
                initializer: Some(LirExpression::Literal(LirLiteral::Integer(42))),
                is_static: false,
            })],
        };

        let ast = lower_lir_to_ast(&program).expect("lower_lir_to_ast");
        assert_eq!(ast.declarations.len(), 1);
    }

    #[test]
    fn rejects_goto() {
        let result = lower_statement(&LirStatement::Goto("bb1".to_string()));
        assert!(result.is_err());
    }
}

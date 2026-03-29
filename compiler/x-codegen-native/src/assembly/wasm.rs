//! Wasm 32 汇编生成器
//!
//! 支持 WebAssembly 文本格式 (WAT).
//!
//! # 支持的 ABI
//!
//! - WebAssembly 标准调用约定
//!
//! # 示例输出
//!
//! ```wat
//! (module
//!   (func $main (result i32)
//!     ;; ... function body ...
//!     i32.const 0
//!     return
//!   )
//!   (export "main" (func $main))
//! )
//! ```

use std::collections::HashMap;
use std::fmt::Write;

use crate::{NativeError, NativeResult, TargetArch, TargetOS};
use x_lir as lir;

use super::AssemblyGenerator;

/// Wasm 32 汇编生成器
pub struct Wasm32AssemblyGenerator {
    /// 目标操作系统
    os: TargetOS,
    /// 输出缓冲区
    output: String,
    /// 当前缩进级别
    indent: usize,
    /// 标签计数器
    label_counter: usize,
    /// 字符串字面量表
    string_literals: HashMap<String, String>,
    /// 全局变量表
    globals: HashMap<String, GlobalInfo>,
    /// 局部变量索引（Wam 使用索引而非栈偏移）
    local_indices: HashMap<String, u32>,
    /// 当前局部变量计数
    local_count: u32,
    /// 当前函数名
    current_function: String,
    /// 循环标签栈 - (continue_label, break_label) for each nested loop
    loop_labels: Vec<(String, String)>,
    /// 字段偏移表 - field name -> calculated offset with alignment (Wasm32 is 32-bit)
    field_offsets: HashMap<String, usize>,
}

/// 全局变量信息
#[allow(dead_code)]
struct GlobalInfo {
    /// 大小
    size: usize,
    /// 是否已初始化
    initialized: bool,
    /// 对齐要求
    align: usize,
}

impl Wasm32AssemblyGenerator {
    /// 创建新的 Wasm 32 汇编生成器
    pub fn new(_os: TargetOS) -> Self {
        Self {
            os: _os,
            output: String::new(),
            indent: 0,
            label_counter: 0,
            string_literals: HashMap::new(),
            globals: HashMap::new(),
            local_indices: HashMap::new(),
            local_count: 0,
            current_function: String::new(),
            loop_labels: Vec::new(),
            field_offsets: HashMap::new(),
        }
    }

    /// 清空生成器状态
    pub fn clear(&mut self) {
        self.output.clear();
        self.indent = 0;
        self.label_counter = 0;
        self.string_literals.clear();
        self.globals.clear();
        self.local_indices.clear();
        self.local_count = 0;
        self.current_function.clear();
        self.loop_labels.clear();
        self.field_offsets.clear();
    }

    /// 生成唯一标签名
    fn new_label(&mut self, prefix: &str) -> String {
        let label = format!("L_{}_{}", prefix, self.label_counter);
        self.label_counter += 1;
        label
    }

    /// 输出一行汇编
    fn emit_line(&mut self, line: &str) -> NativeResult<()> {
        for _ in 0..self.indent {
            self.output.push_str("  ");
        }
        writeln!(self.output, "{}", line)?;
        Ok(())
    }

    /// 输出原始文本
    fn emit_raw(&mut self, text: &str) -> NativeResult<()> {
        for _ in 0..self.indent {
            self.output.push_str("  ");
        }
        writeln!(self.output, "{}", text)?;
        Ok(())
    }

    /// 获取类型的字节大小
    fn size_of_ty(&self, ty: &lir::Type) -> usize {
        match ty {
            lir::Type::Void => 0,
            lir::Type::Bool => 1,
            lir::Type::Char => 4,
            lir::Type::Schar => 1,
            lir::Type::Uchar => 1,
            lir::Type::Short => 2,
            lir::Type::Ushort => 2,
            lir::Type::Int => 4,
            lir::Type::Uint => 4,
            lir::Type::Long => 8,
            lir::Type::Ulong => 8,
            lir::Type::LongLong => 8,
            lir::Type::UlongLong => 8,
            lir::Type::Float => 4,
            lir::Type::Double => 8,
            lir::Type::Size => 4,
            lir::Type::Ptrdiff => 4,
            lir::Type::Pointer(_) => 4,
            lir::Type::FunctionPointer(_, _) => 4,
            _ => 4, // default
        }
    }

    /// 获取类型的对齐要求（字节）
    fn align_of_ty(&self, ty: &lir::Type) -> usize {
        match ty {
            lir::Type::Void => 1,
            lir::Type::Bool => 1,
            lir::Type::Char => 4,
            lir::Type::Schar => 1,
            lir::Type::Uchar => 1,
            lir::Type::Short => 2,
            lir::Type::Ushort => 2,
            lir::Type::Int => 4,
            lir::Type::Uint => 4,
            lir::Type::Long => 8,
            lir::Type::Ulong => 8,
            lir::Type::LongLong => 8,
            lir::Type::UlongLong => 8,
            lir::Type::Float => 4,
            lir::Type::Double => 8,
            lir::Type::Size => 4,
            lir::Type::Ptrdiff => 4,
            lir::Type::Pointer(_) => 4,
            lir::Type::FunctionPointer(_, _) => 4,
            lir::Type::Array(elem, _) => self.align_of_ty(elem),
            lir::Type::Qualified(_, ty) => self.align_of_ty(ty),
            _ => 4,
        }
    }

    /// 将 LIR 类型转换为 Wasm 类型
    fn ty_to_wasm(&self, ty: &lir::Type) -> &'static str {
        match ty {
            lir::Type::Void => "empty",
            lir::Type::Bool => "i32",
            lir::Type::Char => "i32",
            lir::Type::Schar => "i32",
            lir::Type::Uchar => "i32",
            lir::Type::Short => "i32",
            lir::Type::Ushort => "i32",
            lir::Type::Int => "i32",
            lir::Type::Uint => "i32",
            lir::Type::Long => "i64",
            lir::Type::Ulong => "i64",
            lir::Type::LongLong => "i64",
            lir::Type::UlongLong => "i64",
            lir::Type::Float => "f32",
            lir::Type::Double => "f64",
            lir::Type::Size => "i32",
            lir::Type::Ptrdiff => "i32",
            lir::Type::Pointer(_) => "i32",
            lir::Type::FunctionPointer(_, _) => "i32",
            _ => "i32", // default
        }
    }

    /// 加载立即数
    fn emit_load_immediate(&mut self, value: i64, result_reg: &str) -> NativeResult<()> {
        // Wasm 中所有 locals 都是局部变量，结果通过 local.set 获取
        // result_reg 格式: "localN" where N is index
        self.emit_line(&format!("i32.const {}", value))?;
        self.emit_line(&format!("local.set {}", result_reg))?;
        Ok(())
    }

    /// 收集局部变量
    fn collect_locals(stmt: &lir::Statement, locals: &mut Vec<(String, lir::Type)>) {
        match stmt {
            lir::Statement::Variable(var) => {
                locals.push((var.name.clone(), var.type_.clone()));
            }
            lir::Statement::Compound(block) => {
                for stmt in &block.statements {
                    Self::collect_locals(stmt, locals);
                }
            }
            lir::Statement::If(if_stmt) => {
                Self::collect_locals(&*if_stmt.then_branch, locals);
                if let Some(else_branch) = &if_stmt.else_branch {
                    Self::collect_locals(&**else_branch, locals);
                }
            }
            lir::Statement::For(for_stmt) => {
                Self::collect_locals(&*for_stmt.body, locals);
            }
            lir::Statement::While(while_stmt) => {
                Self::collect_locals(&*while_stmt.body, locals);
            }
            lir::Statement::DoWhile(do_while) => {
                Self::collect_locals(&*do_while.body, locals);
            }
            _ => {}
        }
    }

    /// 处理 LIR 表达式，将结果放入指定局部变量
    fn emit_expression(&mut self, expr: &lir::Expression, result_local: &str) -> NativeResult<()> {
        match expr {
            lir::Expression::Literal(lit) => {
                match lit {
                    lir::Literal::Bool(b) => {
                        self.emit_line(&format!("i32.const {}", if *b { 1 } else { 0 }))?;
                        self.emit_line(&format!("local.set {}", result_local))?;
                    }
                    lir::Literal::Integer(i) => {
                        self.emit_load_immediate(*i, result_local)?;
                    }
                    lir::Literal::UnsignedInteger(u) => {
                        self.emit_load_immediate(*u as i64, result_local)?;
                    }
                    lir::Literal::Long(i) => {
                        self.emit_load_immediate(*i, result_local)?;
                    }
                    lir::Literal::UnsignedLong(u) => {
                        self.emit_load_immediate(*u as i64, result_local)?;
                    }
                    lir::Literal::LongLong(i) => {
                        self.emit_load_immediate(*i, result_local)?;
                    }
                    lir::Literal::UnsignedLongLong(u) => {
                        self.emit_load_immediate(*u as i64, result_local)?;
                    }
                    lir::Literal::Float(_f) => {
                        // TODO: 加载浮点数
                        self.emit_line(&format!(";; TODO: float literal"))?;
                        self.emit_line(&format!("i32.const 0"))?;
                        self.emit_line(&format!("local.set {}", result_local))?;
                    }
                    lir::Literal::Double(_d) => {
                        // TODO: 加载浮点数
                        self.emit_line(&format!(";; TODO: double literal"))?;
                        self.emit_line(&format!("i32.const 0"))?;
                        self.emit_line(&format!("local.set {}", result_local))?;
                    }
                    lir::Literal::Char(c) => {
                        self.emit_line(&format!("i32.const {}", *c as i32))?;
                        self.emit_line(&format!("local.set {}", result_local))?;
                    }
                    lir::Literal::String(s) => {
                        // 字符串数据存放在 data 段，将地址加载
                        let label = self.string_literals.get(s).ok_or_else(|| {
                            NativeError::CodegenError(format!("String literal not found: {}", s))
                        })?;
                        self.emit_line(&format!("i32.const {}@addr", label))?;
                        self.emit_line(&format!("local.set {}", result_local))?;
                    }
                    lir::Literal::NullPointer => {
                        self.emit_line(&format!("i32.const 0"))?;
                        self.emit_line(&format!("local.set {}", result_local))?;
                    }
                }
                Ok(())
            }
            lir::Expression::Variable(name) => {
                let idx = self.local_indices.get(name).ok_or_else(|| {
                    NativeError::CodegenError(format!("Variable not found: {}", name))
                })?;
                // Wasm: local.get $name
                self.emit_line(&format!("local.get {}", idx))?;
                self.emit_line(&format!("local.set {}", result_local))?;
                Ok(())
            }
            lir::Expression::Member(base, field_name) => {
                // Get base address (pointer to struct) then add field offset
                let offset_opt = self.field_offsets.get(field_name).copied();
                self.emit_expression(base, "temp_base")?;
                if let Some(offset) = offset_opt {
                    self.emit_line("local.get temp_base")?;
                    self.emit_line(&format!("i32.const {}", offset))?;
                    self.emit_line("i32.add")?;
                    // Now load from the resulting address
                    self.emit_line("i32.load")?;
                    self.emit_line(&format!("local.set {}", result_local))?;
                } else {
                    self.emit_line(&format!(";; TODO: field offset not found: {}", field_name))?;
                    self.emit_line("i32.const 0")?;
                    self.emit_line(&format!("local.set {}", result_local))?;
                }
                Ok(())
            }
            lir::Expression::PointerMember(base, field_name) => {
                // base is already a pointer to the struct, add field offset
                let offset_opt = self.field_offsets.get(field_name).copied();
                self.emit_expression(base, "temp_base")?;
                if let Some(offset) = offset_opt {
                    self.emit_line("local.get temp_base")?;
                    self.emit_line(&format!("i32.const {}", offset))?;
                    self.emit_line("i32.add")?;
                    self.emit_line("i32.load")?;
                    self.emit_line(&format!("local.set {}", result_local))?;
                } else {
                    self.emit_line(&format!(";; TODO: field offset not found: {}", field_name))?;
                    self.emit_line("i32.const 0")?;
                    self.emit_line(&format!("local.set {}", result_local))?;
                }
                Ok(())
            }
            lir::Expression::Dereference(ptr) => {
                self.emit_expression(ptr, "temp")?;
                self.emit_line("i32.load")?;
                self.emit_line(&format!("local.set {}", result_local))?;
                Ok(())
            }
            lir::Expression::AddressOf(expr) => {
                match expr.as_ref() {
                    lir::Expression::Variable(name) => {
                        // Wasm: 局部变量不能直接取地址，全局变量才有地址
                        self.emit_line(&format!(";; TODO: address of local variable {}", name))?;
                        self.emit_line(&format!("i32.const 0"))?;
                        self.emit_line(&format!("local.set {}", result_local))?;
                    }
                    _ => {
                        self.emit_expression(expr, result_local)?;
                    }
                }
                Ok(())
            }
            lir::Expression::Unary(op, operand) => {
                self.emit_expression(operand, "temp")?;
                match op {
                    lir::UnaryOp::Minus => {
                        self.emit_line("i32.const 0")?;
                        self.emit_line("local.get temp")?;
                        self.emit_line("i32.sub")?;
                    }
                    lir::UnaryOp::BitNot => {
                        self.emit_line("i32.const -1")?;
                        self.emit_line("local.get temp")?;
                        self.emit_line("i32.xor")?;
                    }
                    lir::UnaryOp::Not => {
                        self.emit_line("local.get temp")?;
                        self.emit_line("i32.eqz")?;
                    }
                    _ => {
                        self.emit_line(&format!(";; TODO: unary op {:?}", op))?;
                    }
                }
                self.emit_line(&format!("local.set {}", result_local))?;
                Ok(())
            }
            lir::Expression::Binary(op, left, right) => {
                // 先计算左操作数到 temp0，右操作数到 temp1
                self.emit_expression(left, "temp0")?;
                self.emit_expression(right, "temp1")?;

                let wasm_op = match op {
                    lir::BinaryOp::Add => "i32.add",
                    lir::BinaryOp::Subtract => "i32.sub",
                    lir::BinaryOp::Multiply => "i32.mul",
                    lir::BinaryOp::Divide => "i32.div_s",
                    lir::BinaryOp::Modulo => "i32.rem_s",
                    lir::BinaryOp::LeftShift => "i32.shl",
                    lir::BinaryOp::RightShift => "i32.shr_s",
                    lir::BinaryOp::BitAnd => "i32.and",
                    lir::BinaryOp::BitXor => "i32.xor",
                    lir::BinaryOp::BitOr => "i32.or",
                    lir::BinaryOp::LessThan => "i32.lt_s",
                    lir::BinaryOp::LessThanEqual => "i32.le_s",
                    lir::BinaryOp::GreaterThan => "i32.gt_s",
                    lir::BinaryOp::GreaterThanEqual => "i32.ge_s",
                    lir::BinaryOp::Equal => "i32.eq",
                    lir::BinaryOp::NotEqual => "i32.ne",
                    _ => ";; TODO",
                };

                self.emit_line("local.get temp0")?;
                self.emit_line("local.get temp1")?;
                if !matches!(op,
                    lir::BinaryOp::LessThan | lir::BinaryOp::LessThanEqual |
                    lir::BinaryOp::GreaterThan | lir::BinaryOp::GreaterThanEqual |
                    lir::BinaryOp::Equal | lir::BinaryOp::NotEqual) {
                    self.emit_line(wasm_op)?;
                } else {
                    // Comparison operations already produce 0/1 result
                    self.emit_line(wasm_op)?;
                }
                self.emit_line(&format!("local.set {}", result_local))?;
                Ok(())
            }
            lir::Expression::Call(callee, args) => {
                // Wasm ABI: 参数压栈，调用后结果在栈上
                for (i, arg) in args.iter().enumerate() {
                    let temp = format!("temp{}", i);
                    self.emit_expression(arg, &temp)?;
                    self.emit_line(&format!("local.get {}", temp))?;
                }

                match callee.as_ref() {
                    lir::Expression::Variable(name) => {
                        self.emit_line(&format!("call ${}", name))?;
                    }
                    _ => {
                        // 间接调用需要通过 table
                        self.emit_expression(callee, "temp_callee")?;
                        self.emit_line("local.get temp_callee")?;
                        self.emit_line("call_indirect (type $func_ty)")?;
                    }
                }

                // 结果已经在栈上，保存到结果局部变量
                self.emit_line(&format!("local.set {}", result_local))?;
                Ok(())
            }
            lir::Expression::Index(base, index) => {
                self.emit_expression(base, "temp0")?;
                self.emit_expression(index, "temp1")?;
                // Scale by 4 bytes (Wasm 32-bit pointer)
                self.emit_line("local.get temp1")?;
                self.emit_line("i32.const 2")?; // shl by 2 = *4
                self.emit_line("i32.shl")?;
                self.emit_line("local.get temp0")?;
                self.emit_line("i32.add")?;
                self.emit_line("i32.load")?;
                self.emit_line(&format!("local.set {}", result_local))?;
                Ok(())
            }
            lir::Expression::Assign(_, _) => {
                self.emit_line(";; TODO: assignment expression")?;
                self.emit_line("i32.const 0")?;
                self.emit_line(&format!("local.set {}", result_local))?;
                Ok(())
            }
            lir::Expression::AssignOp(_, _, _) => {
                self.emit_line(";; TODO: assignment op expression")?;
                self.emit_line("i32.const 0")?;
                self.emit_line(&format!("local.set {}", result_local))?;
                Ok(())
            }
            lir::Expression::Cast(_ty, expr) => {
                // Wasm: most casts happen via convert instructions
                self.emit_expression(expr, result_local)?;
                // TODO: proper conversion
                Ok(())
            }
            lir::Expression::SizeOf(ty) => {
                let size = self.size_of_ty(ty);
                self.emit_line(&format!("i32.const {}", size))?;
                self.emit_line(&format!("local.set {}", result_local))?;
                Ok(())
            }
            lir::Expression::SizeOfExpr(_expr) => {
                self.emit_line(&format!("i32.const 4"))?;
                self.emit_line(&format!("local.set {}", result_local))?;
                Ok(())
            }
            lir::Expression::AlignOf(ty) => {
                let align = self.align_of_ty(ty);
                self.emit_line(&format!("i32.const {}", align))?;
                self.emit_line(&format!("local.set {}", result_local))?;
                Ok(())
            }
            lir::Expression::Ternary(cond, then, else_) => {
                let else_label = self.new_label("ternary_else");
                let end_label = self.new_label("ternary_end");

                self.emit_expression(cond, "temp_cond")?;
                self.emit_line("local.get temp_cond")?;
                self.emit_line(&format!("br_if {}", else_label))?;

                self.indent += 1;
                self.emit_expression(then, result_local)?;
                self.indent -= 1;

                self.emit_line(&format!("br {}", end_label))?;
                self.emit_raw(&format!("{}:", else_label))?;

                self.indent += 1;
                self.emit_expression(else_, result_local)?;
                self.indent -= 1;

                self.emit_raw(&format!("{}:", end_label))?;
                Ok(())
            }
            lir::Expression::Comma(_) => {
                self.emit_line(";; TODO: comma expression")?;
                self.emit_line("i32.const 0")?;
                self.emit_line(&format!("local.set {}", result_local))?;
                Ok(())
            }
            lir::Expression::Parenthesized(expr) => {
                self.emit_expression(expr, result_local)?;
                Ok(())
            }
            lir::Expression::InitializerList(_) => {
                self.emit_line(";; TODO: initializer list")?;
                self.emit_line("i32.const 0")?;
                self.emit_line(&format!("local.set {}", result_local))?;
                Ok(())
            }
            lir::Expression::CompoundLiteral(_, _) => {
                self.emit_line(";; TODO: compound literal")?;
                self.emit_line("i32.const 0")?;
                self.emit_line(&format!("local.set {}", result_local))?;
                Ok(())
            }
        }
    }

    /// 处理赋值
    #[allow(dead_code)]
    fn emit_assign(&mut self, target: &lir::Expression, source: &lir::Expression) -> NativeResult<()> {
        self.emit_expression(source, "temp")?;

        match target {
            lir::Expression::Variable(name) => {
                let idx = *self.local_indices.get(name).ok_or_else(|| {
                    NativeError::CodegenError(format!("Variable not found: {}", name))
                })?;
                self.emit_line("local.get temp")?;
                self.emit_line(&format!("local.set {}", idx))?;
                Ok(())
            }
            lir::Expression::Member(base, field) => {
                let offset_opt = self.field_offsets.get(field).copied();
                self.emit_expression(base, "t1")?;
                if let Some(offset) = offset_opt {
                    self.emit_line("local.get t1")?;
                    self.emit_line(&format!("i32.const {}", offset))?;
                    self.emit_line("i32.add")?;
                    self.emit_line("local.get temp")?;
                    self.emit_line("i32.store")?;
                } else {
                    self.emit_line(&format!(";; TODO: field offset not found for assignment: {}", field))?;
                }
                Ok(())
            }
            lir::Expression::PointerMember(base, field) => {
                let offset_opt = self.field_offsets.get(field).copied();
                self.emit_expression(base, "t1")?;
                if let Some(offset) = offset_opt {
                    self.emit_line("local.get t1")?;
                    self.emit_line(&format!("i32.const {}", offset))?;
                    self.emit_line("i32.add")?;
                    self.emit_line("local.get temp")?;
                    self.emit_line("i32.store")?;
                } else {
                    self.emit_line(&format!(";; TODO: field offset not found for assignment: {}", field))?;
                }
                Ok(())
            }
            lir::Expression::Dereference(ptr) => {
                self.emit_expression(ptr, "t1")?;
                self.emit_line("local.get temp")?;
                self.emit_line("i32.store")?;
                Ok(())
            }
            lir::Expression::Index(base, idx) => {
                self.emit_expression(base, "t0")?;
                self.emit_expression(idx, "t1")?;
                self.emit_line("local.get t1")?;
                self.emit_line("i32.const 2")?;
                self.emit_line("i32.shl")?;
                self.emit_line("local.get t0")?;
                self.emit_line("i32.add")?;
                self.emit_line("local.get temp")?;
                self.emit_line("i32.store")?;
                Ok(())
            }
            _ => Err(NativeError::CodegenError(format!("Unsupported assignment target: {:?}", target))),
        }
    }

    /// 处理单条语句
    fn emit_statement(&mut self, stmt: &lir::Statement) -> NativeResult<()> {
        match stmt {
            lir::Statement::Empty => Ok(()),
            lir::Statement::Expression(expr) => {
                // Evaluate and drop result
                self.emit_expression(expr, "_")?;
                Ok(())
            }
            lir::Statement::Variable(var) => {
                // 已经在函数开头分配了局部变量索引
                // 如果有初始化表达式，求值并存入
                if let Some(init) = &var.initializer {
                    let idx = self.local_indices[&var.name];
                    self.emit_expression(init, "temp_init")?;
                    self.emit_line(&format!("local.get temp_init"))?;
                    self.emit_line(&format!("local.set {}", idx))?;
                }
                Ok(())
            }
            lir::Statement::If(if_stmt) => {
                let else_label = self.new_label("else");
                let end_label = self.new_label("if_end");

                self.emit_expression(&if_stmt.condition, "temp_cond")?;
                self.emit_line("local.get temp_cond")?;
                self.emit_line(&format!("br_if {}", else_label))?;

                self.indent += 1;
                self.emit_statement(&*if_stmt.then_branch)?;
                self.indent -= 1;

                self.emit_line(&format!("br {}", end_label))?;
                self.emit_raw(&format!("{}:", else_label))?;

                self.indent += 1;
                if let Some(else_branch) = &if_stmt.else_branch {
                    self.emit_statement(&**else_branch)?;
                }
                self.indent -= 1;

                self.emit_raw(&format!("{}:", end_label))?;
                Ok(())
            }
            lir::Statement::For(for_stmt) => {
                let start_label = self.new_label("for_start");
                let end_label = self.new_label("for_end");

                // 初始化语句
                if let Some(init) = &for_stmt.initializer {
                    self.emit_statement(init)?;
                }

                self.emit_raw(&format!("{}:", start_label))?;

                self.indent += 1;
                // 条件检查
                if let Some(cond) = &for_stmt.condition {
                    self.emit_expression(cond, "temp_cond")?;
                    self.emit_line("local.get temp_cond")?;
                    self.emit_line(&format!("br_if {}", end_label))?;
                }

                // 循环体
                self.emit_statement(&*for_stmt.body)?;

                // 增量表达式
                if let Some(inc) = &for_stmt.increment {
                    self.emit_expression(inc, "_")?;
                }

                self.emit_line(&format!("br {}", start_label))?;
                self.indent -= 1;

                self.emit_raw(&format!("{}:", end_label))?;
                Ok(())
            }
            lir::Statement::While(while_stmt) => {
                let start_label = self.new_label("while_start"); // continue jumps here (recheck condition)
                let end_label = self.new_label("while_end");   // break jumps here

                // Push to label stack for break/continue
                self.loop_labels.push((start_label.clone(), end_label.clone()));

                self.emit_raw(&format!("{}:", start_label))?;

                self.indent += 1;
                self.emit_expression(&while_stmt.condition, "temp_cond")?;
                self.emit_line("local.get temp_cond")?;
                self.emit_line(&format!("br_if {}", end_label))?;

                self.emit_statement(&*while_stmt.body)?;
                self.emit_line(&format!("br {}", start_label))?;
                self.indent -= 1;

                self.emit_raw(&format!("{}:", end_label))?;

                // Pop from stack
                self.loop_labels.pop();
                Ok(())
            }
            lir::Statement::DoWhile(do_while) => {
                let start_label = self.new_label("do_start");
                let cond_label = self.new_label("do_cond");
                let end_label = self.new_label("do_end");

                // continue -> go to condition check (then back to start if condition true)
                // break -> exit to end_label
                self.loop_labels.push((cond_label.clone(), end_label.clone()));

                self.emit_raw(&format!("{}:", start_label))?;

                self.indent += 1;
                self.emit_statement(&*do_while.body)?;
                self.emit_line(&format!("br {}", cond_label))?;
                self.emit_raw(&format!("{}:", cond_label))?;
                self.emit_expression(&do_while.condition, "temp_cond")?;
                self.emit_line("local.get temp_cond")?;
                self.emit_line(&format!("br_if {}", start_label))?;
                self.indent -= 1;

                self.emit_raw(&format!("{}:", end_label))?;

                // Pop from stack
                self.loop_labels.pop();
                Ok(())
            }
            lir::Statement::Break => {
                // break jumps to the current loop's break target
                // In Wasm, br is relative to the nesting depth, but since we push/pop labels with the loop nesting, the depth is correct
                if let Some((_continue_label, break_label)) = self.loop_labels.last() {
                    // In Wasm text format, labels are defined with :, so we branch to it
                    self.emit_line(&format!("br {}", break_label))?;
                } else {
                    self.emit_line(";; TODO: break outside loop")?;
                }
                Ok(())
            }
            lir::Statement::Continue => {
                // continue jumps to the current loop's continue target (condition check / loop start)
                if let Some((continue_label, _break_label)) = self.loop_labels.last() {
                    self.emit_line(&format!("br {}", continue_label))?;
                } else {
                    self.emit_line(";; TODO: continue outside loop")?;
                }
                Ok(())
            }
            lir::Statement::Return(expr) => {
                if let Some(expr) = expr {
                    self.emit_expression(expr, "ret_val")?;
                    self.emit_line("local.get ret_val")?;
                }
                self.emit_line("return")?;
                Ok(())
            }
            lir::Statement::Switch(_) => {
                self.emit_line(";; TODO: switch statement")?;
                Ok(())
            }
            lir::Statement::Match(_) => {
                self.emit_line(";; TODO: match statement")?;
                Ok(())
            }
            lir::Statement::Try(_) => {
                self.emit_line(";; TODO: try statement")?;
                Ok(())
            }
            lir::Statement::Goto(label) => {
                self.emit_line(&format!("br {}", label))?;
                Ok(())
            }
            lir::Statement::Label(label) => {
                self.emit_raw(&format!("{}:", label))?;
                Ok(())
            }
            lir::Statement::Declaration(_) => {
                Ok(())
            }
            lir::Statement::Compound(block) => {
                self.indent += 1;
                for stmt in &block.statements {
                    self.emit_statement(stmt)?;
                }
                self.indent -= 1;
                Ok(())
            }
        }
    }

    /// 收集字符串字面量
    fn collect_string_literals(&mut self, expr: &lir::Expression) -> NativeResult<()> {
        match expr {
            lir::Expression::Literal(lir::Literal::String(s)) => {
                if !self.string_literals.contains_key(s) {
                    let label = format!("str_{}", self.string_literals.len());
                    self.string_literals.insert(s.clone(), label);
                }
            }
            lir::Expression::Literal(_) => {}
            lir::Expression::Unary(_, expr) => self.collect_string_literals(expr)?,
            lir::Expression::Binary(_, left, right) => {
                self.collect_string_literals(left)?;
                self.collect_string_literals(right)?;
            }
            lir::Expression::Ternary(c, t, f) => {
                self.collect_string_literals(c)?;
                self.collect_string_literals(t)?;
                self.collect_string_literals(f)?;
            }
            lir::Expression::Assign(target, source) => {
                self.collect_string_literals(target)?;
                self.collect_string_literals(source)?;
            }
            lir::Expression::AssignOp(_, target, source) => {
                self.collect_string_literals(target)?;
                self.collect_string_literals(source)?;
            }
            lir::Expression::Call(callee, args) => {
                self.collect_string_literals(callee)?;
                for arg in args {
                    self.collect_string_literals(arg)?;
                }
            }
            lir::Expression::Index(base, idx) => {
                self.collect_string_literals(base)?;
                self.collect_string_literals(idx)?;
            }
            lir::Expression::Member(base, _field) => self.collect_string_literals(base)?,
            lir::Expression::PointerMember(base, _field) => self.collect_string_literals(base)?,
            lir::Expression::AddressOf(expr) => self.collect_string_literals(expr)?,
            lir::Expression::Dereference(expr) => self.collect_string_literals(expr)?,
            lir::Expression::Cast(_ty, expr) => self.collect_string_literals(expr)?,
            lir::Expression::Comma(exprs) => {
                for expr in exprs {
                    self.collect_string_literals(expr)?;
                }
            }
            lir::Expression::Parenthesized(expr) => self.collect_string_literals(expr)?,
            lir::Expression::InitializerList(inits) => {
                for init in inits {
                    self.collect_string_literals_init(init)?;
                }
            }
            lir::Expression::CompoundLiteral(_ty, inits) => {
                for init in inits {
                    self.collect_string_literals_init(init)?;
                }
            }
            lir::Expression::SizeOf(_) | lir::Expression::SizeOfExpr(_) | lir::Expression::AlignOf(_) => {}
            lir::Expression::Variable(_) => {}
        }
        Ok(())
    }

    /// 收集字符串字面量从初始化器
    fn collect_string_literals_init(&mut self, init: &lir::Initializer) -> NativeResult<()> {
        match init {
            lir::Initializer::Expression(e) => self.collect_string_literals(e)?,
            lir::Initializer::List(list) => {
                for i in list {
                    self.collect_string_literals_init(i)?;
                }
            }
            lir::Initializer::Named(_, i) => self.collect_string_literals_init(i)?,
            lir::Initializer::Indexed(_, i) => self.collect_string_literals_init(i)?,
        }
        Ok(())
    }

    /// 收集字符串字面量从语句
    fn collect_string_literals_stmt(&mut self, stmt: &lir::Statement) -> NativeResult<()> {
        match stmt {
            lir::Statement::Expression(expr) => self.collect_string_literals(expr)?,
            lir::Statement::Variable(var) => {
                if let Some(init) = &var.initializer {
                    self.collect_string_literals(init)?;
                }
            }
            lir::Statement::If(if_stmt) => {
                self.collect_string_literals(&if_stmt.condition)?;
                self.collect_string_literals_stmt(&*if_stmt.then_branch)?;
                if let Some(else_branch) = &if_stmt.else_branch {
                    self.collect_string_literals_stmt(&**else_branch)?;
                }
            }
            lir::Statement::For(for_stmt) => {
                if let Some(init) = &for_stmt.initializer {
                    self.collect_string_literals_stmt(init)?;
                }
                if let Some(cond) = &for_stmt.condition {
                    self.collect_string_literals(cond)?;
                }
                self.collect_string_literals_stmt(&*for_stmt.body)?;
                if let Some(inc) = &for_stmt.increment {
                    self.collect_string_literals(inc)?;
                }
            }
            lir::Statement::While(while_stmt) => {
                self.collect_string_literals(&while_stmt.condition)?;
                self.collect_string_literals_stmt(&*while_stmt.body)?;
            }
            lir::Statement::DoWhile(do_while) => {
                self.collect_string_literals(&do_while.condition)?;
                self.collect_string_literals_stmt(&*do_while.body)?;
            }
            lir::Statement::Compound(block) => {
                for stmt in &block.statements {
                    self.collect_string_literals_stmt(stmt)?;
                }
            }
            lir::Statement::Return(Some(expr)) => self.collect_string_literals(expr)?,
            _ => {}
        }
        Ok(())
    }

    /// 处理一个函数
    fn emit_function(&mut self, func: &lir::Function) -> NativeResult<()> {
        self.clear();
        self.current_function = func.name.clone();

        // 收集所有字符串字面量
        for stmt in &func.body.statements {
            self.collect_string_literals_stmt(stmt)?;
        }

        // 收集所有局部变量
        let mut locals = Vec::new();
        for (_i, param) in func.parameters.iter().enumerate() {
            locals.push((param.name.clone(), param.type_.clone()));
        }
        for stmt in &func.body.statements {
            Self::collect_locals(stmt, &mut locals);
        }

        // 分配局部变量索引
        self.local_indices.clear();
        self.local_count = 0;
        for (name, _ty) in &locals {
            self.local_indices.insert(name.clone(), self.local_count);
            self.local_count += 1;
        }

        // 开始函数定义
        let mut params_wat = String::new();
        for (i, param) in func.parameters.iter().enumerate() {
            let wasm_ty = self.ty_to_wasm(&param.type_);
            if i > 0 {
                params_wat.push(' ');
            }
            params_wat.push_str(&format!("(param ${} {})", param.name, wasm_ty));
        }

        let result_ty = if func.return_type != lir::Type::Void {
            let wasm_ty = self.ty_to_wasm(&func.return_type);
            format!("(result {})", wasm_ty)
        } else {
            String::new()
        };

        self.emit_raw(&format!("(func ${} {} {}", func.name, params_wat, result_ty).trim_end())?;

        // 声明局部变量
        self.indent += 1;
        for (name, ty) in &locals {
            if func.parameters.iter().any(|p| &p.name == name) {
                continue; // parameters already declared in header
            }
            let wasm_ty = self.ty_to_wasm(ty);
            self.emit_line(&format!("(local ${} {})", name, wasm_ty))?;
        }

        // 生成函数体
        self.indent += 1;
        for stmt in &func.body.statements {
            self.emit_statement(stmt)?;
        }
        self.indent -= 1;

        // 如果没有返回语句且非void返回，隐式返回0
        if func.return_type != lir::Type::Void {
            // TODO: check if function already has return
            self.emit_line("i32.const 0")?;
            self.emit_line("return")?;
        }

        self.indent -= 1;
        self.emit_raw(")")?;

        // Export main function
        if func.name == "main" {
            self.emit_line(&format!("(export \"main\" (func ${}))", func.name))?;
        }

        Ok(())
    }
}

impl AssemblyGenerator for Wasm32AssemblyGenerator {
    fn generate(&mut self, lir: &lir::Program) -> NativeResult<String> {
        // Start module
        self.output.clear();
        self.string_literals.clear();
        self.emit_raw("(module")?;
        self.indent += 1;

        // TODO: Type section for indirect calls
        self.emit_line(";; Type definitions for indirect calls")?;
        self.emit_line("(type $func_ty (func))")?;
        self.emit_line("")?;

        // Add memory
        self.emit_line(";; Memory")?;
        self.emit_line("(memory 1)")?;
        self.emit_line("(export \"memory\" (memory 0))")?;
        self.emit_line("")?;

        // First pass: collect all string literals from functions
        for decl in &lir.declarations {
            if let lir::Declaration::Function(func) = decl {
                for stmt in &func.body.statements {
                    self.collect_string_literals_stmt(stmt)?;
                }
            }
        }

        // Calculate struct field offsets with proper alignment
        for decl in &lir.declarations {
            if let lir::Declaration::Struct(struct_decl) = decl {
                let mut current_offset = 0;
                let mut max_align = 1;

                for field in &struct_decl.fields {
                    let align = self.align_of_ty(&field.type_);
                    max_align = max_align.max(align);

                    // Align current offset to field's required alignment
                    current_offset = if current_offset % align == 0 {
                        current_offset
                    } else {
                        current_offset + align - (current_offset % align)
                    };

                    // Store offset - using just the field name (like other architectures)
                    // LIR ensures unique field names after type checking
                    if !self.field_offsets.contains_key(&field.name) {
                        self.field_offsets.insert(field.name.clone(), current_offset);
                    }
                    current_offset += self.size_of_ty(&field.type_);
                }

                // Align the entire struct size to the maximum alignment
                if current_offset % max_align != 0 {
                    current_offset += max_align - (current_offset % max_align);
                }
            }
            // Also handle class declarations
            if let lir::Declaration::Class(class_decl) = decl {
                let mut current_offset = 0;
                let mut max_align = 1;

                for field in &class_decl.fields {
                    let align = self.align_of_ty(&field.type_);
                    max_align = max_align.max(align);

                    // Align current offset to field's required alignment
                    current_offset = if current_offset % align == 0 {
                        current_offset
                    } else {
                        current_offset + align - (current_offset % align)
                    };

                    if !self.field_offsets.contains_key(&field.name) {
                        self.field_offsets.insert(field.name.clone(), current_offset);
                    }
                    current_offset += self.size_of_ty(&field.type_);
                }

                // Align the entire class size to the maximum alignment
                if current_offset % max_align != 0 {
                    current_offset += max_align - (current_offset % max_align);
                }
            }
        }

        // Add string literals to data segment
        if !self.string_literals.is_empty() {
            // Collect to avoid borrow checker issue
            let strings: Vec<(String, String)> = self.string_literals
                .iter()
                .map(|(s, label)| (s.clone(), label.clone()))
                .collect();
            for (s, _label) in strings {
                let escaped = s.escape_default().to_string();
                // Wasm data: (data (i32.const offset) "string")
                // We just place all strings after the static data area
                // TODO: proper offset calculation
                self.emit_line(&format!(";; String: {}", escaped))?;
                self.emit_line(&format!("(data (i32.const 0) \"{}\")", escaped))?;
            }
            self.emit_line("")?;
        }

        // Generate all functions
        for decl in &lir.declarations {
            if let lir::Declaration::Function(func) = decl {
                self.emit_function(func)?;
                self.emit_line("")?;
            }
        }

        // End module
        self.indent -= 1;
        self.emit_raw(")")?;

        Ok(std::mem::take(&mut self.output))
    }

    fn arch(&self) -> TargetArch {
        TargetArch::Wasm32
    }
}

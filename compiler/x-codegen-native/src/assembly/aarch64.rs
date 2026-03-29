//! AArch64 汇编生成器
//!
//! 支持 GNU 汇编语法（Linux/macOS/Windows）。
//!
//! # 支持的 ABI
//!
//! - System V AAPCS ABI (Linux)
//! - Apple ABI (macOS)
//! - Microsoft ARM64 ABI (Windows)
//!
//! # 示例输出
//!
//! ```asm
//! .global main
//! main:
//!     stp x29, x30, [sp, #-16]!
//!     mov x29, sp
//!     sub sp, sp, #32
//!     ; ... function body ...
//!     mov sp, x29
//!     ldp x29, x30, [sp], #16
//!     ret
//! ```

use std::collections::HashMap;
use std::fmt::Write;

use crate::{NativeError, NativeResult, TargetArch, TargetOS};
use x_lir as lir;

use super::AssemblyGenerator;

/// AArch64 汇编生成器
pub struct AArch64AssemblyGenerator {
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
    /// 局部变量栈偏移
    local_offsets: HashMap<String, i32>,
    /// 当前栈帧大小
    stack_size: usize,
    /// 当前函数名
    current_function: String,
    /// 循环标签栈 - (continue_label, break_label) for each nested loop
    loop_labels: Vec<(String, String)>,
    /// 字段偏移表 - field name -> calculated offset with alignment
    field_offsets: HashMap<String, usize>,
}

/// 全局变量信息
#[derive(Debug, Clone)]
struct GlobalInfo {
    size: usize,
    initialized: bool,
    align: usize,
}

impl AArch64AssemblyGenerator {
    /// 创建新的 AArch64 汇编生成器
    pub fn new(os: TargetOS) -> Self {
        Self {
            os,
            output: String::new(),
            indent: 0,
            label_counter: 0,
            string_literals: HashMap::new(),
            globals: HashMap::new(),
            local_offsets: HashMap::new(),
            stack_size: 0,
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
        self.local_offsets.clear();
        self.stack_size = 0;
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
        writeln!(self.output, "{}{}", "    ".repeat(self.indent), line)?;
        Ok(())
    }

    /// 输出原始文本（无缩进）
    fn emit_raw(&mut self, text: &str) -> NativeResult<()> {
        writeln!(self.output, "{}", text)?;
        Ok(())
    }

    /// 生成内存操作数语法
    fn mem_operand(&self, base: &str, offset: i32) -> String {
        if offset == 0 {
            format!("[{}]", base)
        } else if offset & 0xF == 0 {
            format!("[{}, #{:#x}]", base, offset)
        } else {
            format!("[{}, #{}]", base, offset)
        }
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
            lir::Type::LongDouble => 8,
            lir::Type::Size => 8,
            lir::Type::Ptrdiff => 8,
            lir::Type::Intptr => 8,
            lir::Type::Uintptr => 8,
            lir::Type::Pointer(_) => 8,
            lir::Type::Array(ty, len) => {
                let len = len.unwrap_or(1);
                self.size_of_ty(ty) * (len as usize)
            }
            lir::Type::FunctionPointer(_, _) => 8,
            lir::Type::Named(_) => 8,
            lir::Type::Qualified(_, ty) => self.size_of_ty(ty),
        }
    }

    /// 将整数加载到寄存器
    fn emit_load_immediate(&mut self, value: i64, reg: &str) -> NativeResult<()> {
        if value >= 0 && value <= 0xFFFF {
            self.emit_line(&format!("mov {}, #{}", reg, value))?;
        } else {
            // 对于大立即数，需要多步加载
            let low = value & 0xFFFF;
            let high = (value >> 16) & 0xFFFF;
            if high != 0 {
                self.emit_line(&format!("movz {}, #{}", reg, low))?;
                self.emit_line(&format!("movk {}, #{} lsl 16", reg, high))?;
                let high2 = (value >> 32) & 0xFFFF;
                if high2 != 0 {
                    self.emit_line(&format!("movk {}, #{} lsl 32", reg, high2))?;
                }
                let high3 = (value >> 48) & 0xFFFF;
                if high3 != 0 {
                    self.emit_line(&format!("movk {}, #{} lsl 48", reg, high3))?;
                }
            } else {
                self.emit_line(&format!("mov {}, #{}", reg, low))?;
            }
        }
        Ok(())
    }

    /// 处理 LIR 表达式，将结果放入指定寄存器
    fn emit_expression(&mut self, expr: &lir::Expression, result_reg: &str) -> NativeResult<()> {
        match expr {
            lir::Expression::Literal(lit) => {
                match lit {
                    lir::Literal::Bool(b) => {
                        self.emit_line(&format!("mov {}, #{}", result_reg, if *b { 1 } else { 0 }))?;
                    }
                    lir::Literal::Integer(i) => {
                        self.emit_load_immediate(*i, result_reg)?;
                    }
                    lir::Literal::UnsignedInteger(u) => {
                        self.emit_load_immediate(*u as i64, result_reg)?;
                    }
                    lir::Literal::Long(i) => {
                        self.emit_load_immediate(*i, result_reg)?;
                    }
                    lir::Literal::UnsignedLong(u) => {
                        self.emit_load_immediate(*u as i64, result_reg)?;
                    }
                    lir::Literal::LongLong(i) => {
                        self.emit_load_immediate(*i, result_reg)?;
                    }
                    lir::Literal::UnsignedLongLong(u) => {
                        self.emit_load_immediate(*u as i64, result_reg)?;
                    }
                    lir::Literal::Float(f) => {
                        // TODO: 加载浮点数到 SIMD 寄存器
                        self.emit_line(&format!("// TODO: float literal {}", f))?;
                    }
                    lir::Literal::Double(f) => {
                        // TODO: 加载浮点数到 SIMD 寄存器
                        self.emit_line(&format!("// TODO: double literal {}", f))?;
                    }
                    lir::Literal::Char(c) => {
                        self.emit_line(&format!("mov {}, #{}", result_reg, *c as i64))?;
                    }
                    lir::Literal::String(s) => {
                        // 字符串字面量存在 rodata，将地址加载到寄存器
                        let label = self.string_literals.get(s).ok_or_else(|| {
                            NativeError::CodegenError(format!("String literal not found: {}", s))
                        })?;
                        match self.os {
                            TargetOS::MacOS => {
                                self.emit_line(&format!("adrp {}, :got:{}", result_reg, label))?;
                                self.emit_line(&format!("ldr {}, [{}]", result_reg, result_reg))?;
                            }
                            _ => {
                                self.emit_line(&format!("ldr {}, ={}", result_reg, label))?;
                            }
                        }
                    }
                    lir::Literal::NullPointer => {
                        self.emit_line(&format!("mov {}, #0", result_reg))?;
                    }
                }
                Ok(())
            }
            lir::Expression::Variable(name) => {
                let offset = self.local_offsets.get(name).ok_or_else(|| {
                    NativeError::CodegenError(format!("Variable not found: {}", name))
                })?;
                self.emit_line(&format!("ldr {}, {}", result_reg, self.mem_operand("x29", *offset)))?;
                Ok(())
            }
            lir::Expression::Member(base, field) => {
                self.emit_expression(base, result_reg)?;
                // Add field offset if available
                if let Some(&offset) = self.field_offsets.get(field) {
                    if offset > 0 {
                        self.emit_line(&format!("add {}, {}, #{}", result_reg, result_reg, offset))?;
                    }
                }
                self.emit_line(&format!("ldr {}, [{}]", result_reg, result_reg))?;
                Ok(())
            }
            lir::Expression::PointerMember(base, field) => {
                self.emit_expression(base, result_reg)?;
                self.emit_line(&format!("ldr {}, [{}]", result_reg, result_reg))?;
                // Add field offset if available
                if let Some(&offset) = self.field_offsets.get(field) {
                    if offset > 0 {
                        self.emit_line(&format!("add {}, {}, #{}", result_reg, result_reg, offset))?;
                    }
                }
                self.emit_line(&format!("ldr {}, [{}]", result_reg, result_reg))?;
                Ok(())
            }
            lir::Expression::Dereference(ptr) => {
                self.emit_expression(ptr, result_reg)?;
                self.emit_line(&format!("ldr {}, [{}]", result_reg, result_reg))?;
                Ok(())
            }
            lir::Expression::AddressOf(expr) => {
                match expr.as_ref() {
                    lir::Expression::Variable(name) => {
                        let offset = self.local_offsets.get(name).ok_or_else(|| {
                            NativeError::CodegenError(format!("Variable not found: {}", name))
                        })?;
                        self.emit_line(&format!("add {}, x29, #{}", result_reg, offset))?;
                    }
                    lir::Expression::Member(base, field) => {
                        self.emit_expression(base, result_reg)?;
                        if let Some(&offset) = self.field_offsets.get(field) {
                            if offset > 0 {
                                self.emit_line(&format!("add {}, {}, #{}", result_reg, result_reg, offset))?;
                            }
                        }
                    }
                    lir::Expression::PointerMember(base, field) => {
                        self.emit_expression(base, result_reg)?;
                        self.emit_line(&format!("ldr {}, [{}]", result_reg, result_reg))?;
                        if let Some(&offset) = self.field_offsets.get(field) {
                            if offset > 0 {
                                self.emit_line(&format!("add {}, {}, #{}", result_reg, result_reg, offset))?;
                            }
                        }
                    }
                    _ => {
                        // TODO: 处理其他情况
                        self.emit_expression(expr, result_reg)?;
                    }
                }
                Ok(())
            }
            lir::Expression::Unary(op, operand) => {
                self.emit_expression(operand, "x9")?;
                match op {
                    lir::UnaryOp::Minus => {
                        self.emit_line(&format!("neg {}, x9", result_reg))?;
                    }
                    lir::UnaryOp::BitNot => {
                        self.emit_line(&format!("mvn {}, x9", result_reg))?;
                    }
                    lir::UnaryOp::Not => {
                        // Logical not - 结果为 0 或 1
                        self.emit_line("cmp x9, #0")?;
                        self.emit_line(&format!("cset {}, eq", result_reg))?;
                    }
                    _ => {
                        // TODO: increment/decrement
                        self.emit_line(&format!("// TODO: unary op {:?}", op))?;
                    }
                }
                Ok(())
            }
            lir::Expression::Binary(op, left, right) => {
                // 先计算左操作数到 x9，右操作数到 x10
                self.emit_expression(left, "x9")?;
                self.emit_expression(right, "x10")?;

                let need_cset = matches!(op,
                    lir::BinaryOp::LessThan | lir::BinaryOp::LessThanEqual |
                    lir::BinaryOp::GreaterThan | lir::BinaryOp::GreaterThanEqual |
                    lir::BinaryOp::Equal | lir::BinaryOp::NotEqual);

                let op_name = match op {
                    lir::BinaryOp::Add => "add",
                    lir::BinaryOp::Subtract => "sub",
                    lir::BinaryOp::Multiply => "mul",
                    lir::BinaryOp::Divide => "sdiv",
                    lir::BinaryOp::Modulo => "srem",
                    lir::BinaryOp::LeftShift => "lsl",
                    lir::BinaryOp::RightShift => "lsr",
                    lir::BinaryOp::RightShiftArithmetic => "asr",
                    lir::BinaryOp::BitAnd => "and",
                    lir::BinaryOp::BitXor => "eor",
                    lir::BinaryOp::BitOr => "orr",
                    lir::BinaryOp::LessThan => "cmp",
                    lir::BinaryOp::LessThanEqual => "cmp",
                    lir::BinaryOp::GreaterThan => "cmp",
                    lir::BinaryOp::GreaterThanEqual => "cmp",
                    lir::BinaryOp::Equal => "cmp",
                    lir::BinaryOp::NotEqual => "cmp",
                    _ => "// TODO",
                };

                if !need_cset {
                    self.emit_line(&format!("{} {}, x9, x10", op_name, result_reg))?;
                } else {
                    self.emit_line(&format!("{} x9, x10", op_name))?;
                    let cset_cc = match op {
                        lir::BinaryOp::Equal => "eq",
                        lir::BinaryOp::NotEqual => "ne",
                        lir::BinaryOp::LessThan => "lt",
                        lir::BinaryOp::LessThanEqual => "le",
                        lir::BinaryOp::GreaterThan => "gt",
                        lir::BinaryOp::GreaterThanEqual => "ge",
                        _ => unreachable!(),
                    };
                    self.emit_line(&format!("cset {}, {}", result_reg, cset_cc))?;
                }

                Ok(())
            }
            lir::Expression::Call(callee, args) => {
                // AAPCS: X0-X7 用于参数，结果在 X0
                // 先将参数加载到寄存器
                for (i, arg) in args.iter().enumerate() {
                    if i < 8 {
                        let arg_reg = format!("x{}", i);
                        self.emit_expression(arg, &arg_reg)?;
                    } else {
                        // TODO: 栈参数
                        self.emit_line(&format!("// TODO: parameter {} on stack", i))?;
                    }
                }

                // 调用函数：直接调用或间接调用
                match callee.as_ref() {
                    lir::Expression::Variable(name) => {
                        // 直接调用命名函数
                        self.emit_line(&format!("bl {}", name))?;
                    }
                    _ => {
                        // 间接调用，函数指针在表达式结果中
                        self.emit_expression(callee, "x9")?;
                        self.emit_line("blr x9")?;
                    }
                }

                // 结果已经在 x0，移动到目标寄存器
                if result_reg != "x0" {
                    self.emit_line(&format!("mov {}, x0", result_reg))?;
                }
                Ok(())
            }
            lir::Expression::Index(base, index) => {
                self.emit_expression(base, "x10")?;
                self.emit_expression(index, "x11")?;
                // Scale by 8 bytes (assuming 64-bit)
                self.emit_line("lsl x11, x11, #3")?;
                self.emit_line("add x10, x10, x11")?;
                self.emit_line(&format!("ldr {}, [x10]", result_reg))?;
                Ok(())
            }
            lir::Expression::Assign(_, _) => {
                // Assignment as expression - TODO
                self.emit_line("// TODO: assignment expression")?;
                Ok(())
            }
            lir::Expression::AssignOp(_, _, _) => {
                // Assign operation as expression - TODO
                self.emit_line("// TODO: assignment op expression")?;
                Ok(())
            }
            lir::Expression::Cast(_ty, expr) => {
                // AArch64 大部分 cast 直接通过寄存器传递即可
                self.emit_expression(expr, result_reg)?;
                Ok(())
            }
            lir::Expression::SizeOf(ty) => {
                let size = self.size_of_ty(ty);
                self.emit_line(&format!("mov {}, #{}", result_reg, size))?;
                Ok(())
            }
            lir::Expression::SizeOfExpr(_expr) => {
                // TODO: 计算表达式类型大小
                self.emit_line(&format!("mov {}, #8", result_reg))?;
                Ok(())
            }
            lir::Expression::AlignOf(ty) => {
                let align = self.type_align(ty);
                self.emit_line(&format!("mov {}, #{}", result_reg, align))?;
                Ok(())
            }
            lir::Expression::Ternary(cond, then, else_) => {
                let else_label = self.new_label("ternary_else");
                let end_label = self.new_label("ternary_end");

                self.emit_expression(cond, result_reg)?;
                self.emit_line(&format!("cmp {}, #0", result_reg))?;
                self.emit_line(&format!("b.eq {}", else_label))?;

                self.emit_expression(then, result_reg)?;
                self.emit_line(&format!("b {}", end_label))?;

                self.emit_raw(&format!("{}:", else_label))?;
                self.emit_expression(else_, result_reg)?;

                self.emit_raw(&format!("{}:", end_label))?;
                Ok(())
            }
            lir::Expression::Comma(exprs) => {
                // Comma expression: evaluate all expressions in order,
                // result is the value of the last one
                for (i, expr) in exprs.iter().enumerate() {
                    if i == exprs.len() - 1 {
                        // Last one - result goes to result_reg
                        self.emit_expression(expr, result_reg)?;
                    } else {
                        // Evaluate but discard result
                        self.emit_expression(expr, "x9")?;
                    }
                }
                Ok(())
            }
            lir::Expression::Parenthesized(expr) => {
                self.emit_expression(expr, result_reg)?;
                Ok(())
            }
            lir::Expression::InitializerList(_) => {
                // TODO: initializer list
                self.emit_line("// TODO: initializer list")?;
                Ok(())
            }
            lir::Expression::CompoundLiteral(_, _) => {
                // TODO: compound literal
                self.emit_line("// TODO: compound literal")?;
                Ok(())
            }
        }
    }

    /// 处理赋值语句 (Statement::Assign)
    fn emit_assign(&mut self, target: &lir::Expression, source: &lir::Expression) -> NativeResult<()> {
        // 先计算源表达式的值到 x9
        self.emit_expression(source, "x9")?;

        // 然后存储到目标位置
        match target {
            lir::Expression::Variable(name) => {
                let offset = self.local_offsets.get(name).ok_or_else(|| {
                    NativeError::CodegenError(format!("Variable not found: {}", name))
                })?;
                self.emit_line(&format!("str x9, {}", self.mem_operand("x29", *offset)))?;
                Ok(())
            }
            lir::Expression::Member(base, field) => {
                self.emit_expression(base, "x10")?;
                // Add field offset if available
                if let Some(&offset) = self.field_offsets.get(field) {
                    if offset > 0 {
                        // AArch64 can do offset in the load/store instruction for small immediates
                        if offset <= 4095 {
                            self.emit_line(&format!("str x9, [x10, #{}, lsl #0]", offset))?;
                        } else {
                            // For large offsets, add first then store
                            self.emit_line(&format!("add x10, x10, #{}", offset))?;
                            self.emit_line("str x9, [x10]")?;
                        }
                    } else {
                        self.emit_line("str x9, [x10]")?;
                    }
                } else {
                    // Field not found - store directly (offset 0)
                    self.emit_line("str x9, [x10]")?;
                }
                Ok(())
            }
            lir::Expression::PointerMember(base, field) => {
                self.emit_expression(base, "x10")?;
                self.emit_line("ldr x10, [x10]")?;
                // Add field offset if available
                if let Some(&offset) = self.field_offsets.get(field) {
                    if offset > 0 {
                        if offset <= 4095 {
                            self.emit_line(&format!("str x9, [x10, #{}, lsl #0]", offset))?;
                        } else {
                            self.emit_line(&format!("add x10, x10, #{}", offset))?;
                            self.emit_line("str x9, [x10]")?;
                        }
                    } else {
                        self.emit_line("str x9, [x10]")?;
                    }
                } else {
                    // Field not found - store directly (offset 0)
                    self.emit_line("str x9, [x10]")?;
                }
                Ok(())
            }
            lir::Expression::Dereference(ptr) => {
                self.emit_expression(ptr, "x10")?;
                self.emit_line("str x9, [x10]")?;
                Ok(())
            }
            lir::Expression::Index(base, index) => {
                self.emit_expression(base, "x10")?;
                self.emit_expression(index, "x11")?;
                // Scale by 8 bytes (assuming 64-bit)
                self.emit_line("lsl x11, x11, #3")?;
                self.emit_line("add x10, x10, x11")?;
                self.emit_line("str x9, [x10]")?;
                Ok(())
            }
            _ => {
                // TODO: 其他目标类型
                Err(NativeError::CodegenError(format!("Unsupported assignment target: {:?}", target)))
            }
        }
    }

    /// 处理单条语句
    fn emit_statement(&mut self, stmt: &lir::Statement) -> NativeResult<()> {
        match stmt {
            lir::Statement::Empty => Ok(()),
            lir::Statement::Expression(expr) => {
                // 计算表达式但丢弃结果
                self.emit_expression(expr, "x0")?;
                Ok(())
            }
            lir::Statement::Variable(var) => {
                // 局部变量声明已经在函数序言处理过
                // 如果有初始化表达式，对它求值并存储
                if let Some(init) = &var.initializer {
                    let offset = self.local_offsets[&var.name];
                    self.emit_expression(init, "x9")?;
                    self.emit_line(&format!("str x9, {}", self.mem_operand("x29", offset)))?;
                }
                Ok(())
            }
            lir::Statement::If(if_stmt) => {
                let else_label = self.new_label("else");
                let end_label = self.new_label("if_end");

                self.emit_expression(&if_stmt.condition, "x9")?;
                self.emit_line("cmp x9, #0")?;
                self.emit_line(&format!("b.eq {}", else_label))?;

                self.indent += 1;
                self.emit_statement(&*if_stmt.then_branch)?;
                self.indent -= 1;

                self.emit_line(&format!("b {}", end_label))?;
                self.emit_raw(&format!("{}:", else_label))?;

                self.indent += 1;
                if let Some(else_branch) = &if_stmt.else_branch {
                    self.emit_statement(&**else_branch)?;
                }
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
                self.emit_expression(&while_stmt.condition, "x9")?;
                self.emit_line("cmp x9, #0")?;
                self.emit_line(&format!("b.eq {}", end_label))?;

                self.emit_statement(&*while_stmt.body)?;
                self.emit_line(&format!("b {}", start_label))?;
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
                self.emit_line(&format!("b {}", cond_label))?;
                self.emit_raw(&format!("{}:", cond_label))?;
                self.emit_expression(&do_while.condition, "x9")?;
                self.emit_line("cmp x9, #0")?;
                self.emit_line(&format!("b.ne {}", start_label))?;
                self.indent -= 1;

                self.emit_raw(&format!("{}:", end_label))?;

                // Pop from stack
                self.loop_labels.pop();
                Ok(())
            }
            lir::Statement::For(for_stmt) => {
                let start_label = self.new_label("for_start");
                let end_label = self.new_label("for_end");
                // continue -> go to condition check (then loop if true)
                // break -> exit to end_label

                // initializer
                if let Some(init) = &for_stmt.initializer {
                    self.emit_statement(init)?;
                }

                // Push to label stack
                self.loop_labels.push((start_label.clone(), end_label.clone()));

                self.emit_raw(&format!("{}:", start_label))?;

                // condition check
                self.indent += 1;
                if let Some(cond) = &for_stmt.condition {
                    self.emit_expression(cond, "x9")?;
                    self.emit_line("cmp x9, #0")?;
                    self.emit_line(&format!("b.eq {}", end_label))?;
                }

                // loop body
                self.emit_statement(&for_stmt.body)?;

                // increment
                if let Some(inc) = &for_stmt.increment {
                    self.emit_expression(inc, "x9")?;
                }

                self.emit_line(&format!("b {}", start_label))?;
                self.indent -= 1;

                self.emit_raw(&format!("{}:", end_label))?;

                // Pop from stack
                self.loop_labels.pop();
                Ok(())
            }
            lir::Statement::Break => {
                // break jumps to the current loop's break target
                if let Some((_continue_label, break_label)) = self.loop_labels.last() {
                    self.emit_line(&format!("b {}", break_label))?;
                } else {
                    self.emit_line("// TODO: break outside loop")?;
                }
                Ok(())
            }
            lir::Statement::Continue => {
                // continue jumps to the current loop's continue target (condition check / loop start)
                if let Some((continue_label, _break_label)) = self.loop_labels.last() {
                    self.emit_line(&format!("b {}", continue_label))?;
                } else {
                    self.emit_line("// TODO: continue outside loop")?;
                }
                Ok(())
            }
            lir::Statement::Return(expr) => {
                if let Some(expr) = expr {
                    self.emit_expression(expr, "x0")?;
                }
                // 恢复栈帧并返回
                self.emit_line("mov sp, x29")?;
                self.emit_line("ldp x29, x30, [sp], #16")?;
                self.emit_line("ret")?;
                Ok(())
            }
            lir::Statement::Switch(_) => {
                // TODO: 实现 switch
                self.emit_line("// TODO: switch statement")?;
                Ok(())
            }
            lir::Statement::Match(_) => {
                // TODO: 实现 match
                self.emit_line("// TODO: match statement")?;
                Ok(())
            }
            lir::Statement::Try(_) => {
                // TODO: 实现 try
                self.emit_line("// TODO: try statement")?;
                Ok(())
            }
            lir::Statement::Goto(label) => {
                self.emit_line(&format!("b {}", label))?;
                Ok(())
            }
            lir::Statement::Label(label) => {
                self.emit_raw(&format!("{}:", label))?;
                Ok(())
            }
            lir::Statement::Declaration(_) => {
                // 声明已经在函数开头处理
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

    /// 处理一个函数
    fn emit_function(&mut self, func: &lir::Function) -> NativeResult<()> {
        self.clear();
        self.current_function = func.name.clone();

        // 收集所有局部变量从函数体
        // 遍历语句找出所有 Variable 声明
        let mut locals = Vec::new();
        fn collect_locals(stmt: &lir::Statement, locals: &mut Vec<(String, lir::Type)>) {
            match stmt {
                lir::Statement::Variable(var) => {
                    locals.push((var.name.clone(), var.type_.clone()));
                }
                lir::Statement::Compound(block) => {
                    for stmt in &block.statements {
                        collect_locals(stmt, locals);
                    }
                }
                lir::Statement::If(if_stmt) => {
                    collect_locals(&*if_stmt.then_branch, locals);
                    if let Some(else_branch) = &if_stmt.else_branch {
                        collect_locals(&**else_branch, locals);
                    }
                }
                lir::Statement::While(while_stmt) => {
                    collect_locals(&*while_stmt.body, locals);
                }
                lir::Statement::DoWhile(do_while) => {
                    collect_locals(&*do_while.body, locals);
                }
                _ => {}
            }
        }
        for stmt in &func.body.statements {
            collect_locals(stmt, &mut locals);
        }

        // 声明全局符号
        match self.os {
            TargetOS::MacOS => {
                self.emit_raw(&format!(".global _{}", func.name))?;
                self.emit_raw(&format!("_{}:", func.name))?;
            }
            _ => {
                self.emit_raw(&format!(".global {}", func.name))?;
                self.emit_raw(&format!(".type {}, %function", func.name))?;
                self.emit_raw(&format!("{}:", func.name))?;
            }
        }

        // 函数序言：保存帧指针和链接寄存器
        self.emit_line("stp x29, x30, [sp, #-16]!")?;
        self.emit_line("mov x29, sp")?;

        // 计算栈帧大小并分配栈空间
        // 计算所有局部变量的总大小并对齐到 16 字节
        let mut total_stack = 0usize;
        for (_, ty) in &locals {
            let size = self.size_of_ty(ty);
            let align = if size >= 8 { 8 } else { size };
            total_stack = (total_stack + size + align - 1) & !(align - 1);
        }
        self.stack_size = (total_stack + 15) & !15; // 16 字节对齐

        if self.stack_size > 0 {
            self.emit_line(&format!("sub sp, sp, #{}", self.stack_size))?;
        }

        // 计算局部变量的栈偏移（x29 指向帧指针，从负偏移向下增长）
        let mut current_offset: i32 = -16; // x29 - 16 开始（已经保存了 x29, x30）
        for (name, ty) in &locals {
            let size = self.size_of_ty(&ty);
            let align = if size >= 8 { 8 } else { size };
            // 对齐调整
            let adjustment = ((size + align - 1) & !(align - 1)) as i32;
            current_offset -= adjustment;
            self.local_offsets.insert(name.clone(), current_offset);
        }

        // 将参数从参数寄存器(x0-x7)存储到栈上的局部变量
        // 参数按照顺序分配给前 8 个参数位置
        for (i, param) in func.parameters.iter().enumerate() {
            if i < 8 {
                let offset = self.local_offsets[&param.name];
                self.emit_line(&format!("str x{}, {}", i, self.mem_operand("x29", offset)))?;
            } else {
                // 参数超过 8 个已经在栈上，从 x29 + 16 开始读取
                // TODO: 处理超过 8 个参数的情况
                self.emit_line(&format!("// TODO: parameter {} on stack", i))?;
            }
        }

        // 发射函数体
        self.indent += 1;
        for stmt in &func.body.statements {
            self.emit_statement(stmt)?;
        }
        self.indent -= 1;

        // 如果函数没有显式返回，我们需要添加一个默认返回
        if let Some(last) = func.body.statements.last() {
            if !matches!(last, lir::Statement::Return(_)) {
                self.emit_line("mov sp, x29")?;
                self.emit_line("ldp x29, x30, [sp], #16")?;
                self.emit_line("ret")?;
            }
        } else {
            // 空函数，直接返回
            self.emit_line("mov sp, x29")?;
            self.emit_line("ldp x29, x30, [sp], #16")?;
            self.emit_line("ret")?;
        }

        // 函数大小声明
        if matches!(self.os, TargetOS::Linux | TargetOS::MacOS) {
            self.emit_raw(&format!(".size {}, .-{}", func.name, func.name))?;
        }
        self.emit_raw("")?;

        Ok(())
    }

    /// 获取类型大小（字节）- AArch64 ABI
    fn type_size(&self, ty: &lir::Type) -> usize {
        use lir::Type;
        match ty {
            Type::Void => 0,
            Type::Bool => 1,
            Type::Char | Type::Schar | Type::Uchar => 1,
            Type::Short | Type::Ushort => 2,
            Type::Int | Type::Uint => 4,
            Type::Long | Type::Ulong => 8,
            Type::LongLong | Type::UlongLong => 8,
            Type::Float => 4,
            Type::Double => 8,
            Type::LongDouble => 16,
            Type::Size | Type::Ptrdiff | Type::Intptr | Type::Uintptr => 8,
            Type::Pointer(_) => 8,
            Type::FunctionPointer(_, _) => 8,
            Type::Array(elem, Some(len)) => elem.size_of() * (*len as usize),
            Type::Array(elem, None) => elem.size_of(),
            Type::Named(_) => 0,
            Type::Qualified(_, ty) => ty.size_of(),
        }
    }

    /// 获取类型对齐（字节）- AArch64 ABI
    fn type_align(&self, ty: &lir::Type) -> usize {
        use lir::Type;
        match ty {
            Type::Void => 1,
            Type::Bool => 1,
            Type::Char | Type::Schar | Type::Uchar => 1,
            Type::Short | Type::Ushort => 2,
            Type::Int | Type::Uint => 4,
            Type::Long | Type::Ulong => 8,
            Type::LongLong | Type::UlongLong => 8,
            Type::Float => 4,
            Type::Double => 8,
            Type::LongDouble => 16,
            Type::Size | Type::Ptrdiff | Type::Intptr | Type::Uintptr => 8,
            Type::Pointer(_) => 8,
            Type::FunctionPointer(_, _) => 8,
            Type::Array(elem, _) => elem.align_of(),
            Type::Named(_) => 8,
            Type::Qualified(_, ty) => ty.align_of(),
        }
    }
}

impl AssemblyGenerator for AArch64AssemblyGenerator {
    fn generate(&mut self, lir: &lir::Program) -> NativeResult<String> {
        self.clear();

        // Calculate field offsets for all structs and classes
        for decl in &lir.declarations {
            match decl {
                lir::Declaration::Struct(strct) => {
                    // Calculate field offsets with proper alignment
                    let mut current_offset = 0;
                    let mut max_alignment = 1;

                    for field in &strct.fields {
                        let align = self.type_align(&field.type_);
                        max_alignment = max_alignment.max(align);

                        // Align current offset to field alignment requirement
                        if current_offset % align != 0 {
                            current_offset += align - (current_offset % align);
                        }

                        // Store offset - first occurrence wins
                        if !self.field_offsets.contains_key(&field.name) {
                            let size = self.type_size(&field.type_);
                            self.field_offsets.insert(field.name.clone(), current_offset);
                            current_offset += size;
                        }
                    }

                    // Align total struct size
                    if current_offset % max_alignment != 0 {
                        current_offset += max_alignment - (current_offset % max_alignment);
                    }
                }
                lir::Declaration::Class(cls) => {
                    // Calculate field offsets with proper alignment - same as struct
                    let mut current_offset = 0;
                    let mut max_alignment = 1;

                    for field in &cls.fields {
                        let align = self.type_align(&field.type_);
                        max_alignment = max_alignment.max(align);

                        // Align current offset to field alignment requirement
                        if current_offset % align != 0 {
                            current_offset += align - (current_offset % align);
                        }

                        // Store offset - first occurrence wins
                        if !self.field_offsets.contains_key(&field.name) {
                            let size = self.type_size(&field.type_);
                            self.field_offsets.insert(field.name.clone(), current_offset);
                            current_offset += size;
                        }
                    }

                    // Align total class size
                    if current_offset % max_alignment != 0 {
                        current_offset += max_alignment - (current_offset % max_alignment);
                    }
                }
                lir::Declaration::Global(global) => {
                    let size = self.type_size(&global.type_);
                    self.globals.insert(
                        global.name.clone(),
                        GlobalInfo {
                            size,
                            initialized: global.initializer.is_some(),
                            align: self.type_align(&global.type_),
                        },
                    );
                }
                lir::Declaration::Function(func) => {
                    // Collect string literals from function body
                    for stmt in &func.body.statements {
                        // TODO: recursively collect strings
                    }
                }
                _ => {}
            }
        }

        // 文件开头 - 定义段
        self.emit_raw(".text")?;
        self.emit_raw("")?;

        // 生成所有函数
        for decl in &lir.declarations {
            if let lir::Declaration::Function(func) = decl {
                self.emit_function(func)?;
            }
        }

        // 生成字符串字面量只读数据段
        if !self.string_literals.is_empty() {
            self.emit_raw("")?;
            self.emit_raw(".section .rodata")?;
            let strings: Vec<(String, String)> = self.string_literals.iter()
                .map(|(s, l)| (s.clone(), l.clone()))
                .collect();
            for (s, label) in strings {
                self.emit_raw(&format!(".align 3"))?;
                self.emit_raw(&format!("{}:", label))?;
                self.indent += 1;
                self.emit_line(&format!(".asciz \"{}\"", s.escape_debug()))?;
                self.indent -= 1;
            }
        }

        // 生成全局变量
        if !self.globals.is_empty() {
            self.emit_raw("")?;
            self.emit_raw(".data")?;
            let globals: Vec<(String, GlobalInfo)> = self.globals.iter()
                .map(|(n, i)| (n.clone(), i.clone()))
                .collect();
            for (name, info) in globals {
                self.emit_raw(&format!(".align {}", info.align.trailing_zeros()))?;
                self.emit_raw(&format!(".global {}", name))?;
                self.emit_raw(&format!("{}:", name))?;
                self.indent += 1;
                if info.initialized {
                    self.emit_line(&format!(".space {}", info.size))?;
                } else {
                    self.emit_line(&format!(".zero {}", info.size))?;
                }
                self.indent -= 1;
            }
        }

        Ok(self.output.clone())
    }

    fn arch(&self) -> crate::TargetArch {
        crate::TargetArch::AArch64
    }
}

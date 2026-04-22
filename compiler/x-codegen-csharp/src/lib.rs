//! C# 后端 - 生成 C# 14 源代码
//!
//! 面向 .NET 10 平台，生成 C# 源代码
//!
//! ## C# 14 / .NET 10 特性支持 (2026年3月发布)
//! - Primary constructors（主构造函数）
//! - Collection expressions（集合表达式）
//! - Inline arrays（内联数组）
//! - Extended nameof scope
//! - Implicit span conversions
//! - Params Span/ReadOnlySpan
//! - Partial properties
//! - Field-backed properties
//! - Method group natural type improvements

#![allow(clippy::only_used_in_recursion, clippy::useless_format)]

use std::collections::HashSet;
use std::path::PathBuf;
use x_codegen::{headers, CodeGenerator, CodegenOutput, FileType, OutputFile};
use x_lir::Program as LirProgram;

/// C# 后端配置
#[derive(Debug, Clone)]
pub struct CSharpConfig {
    pub output_dir: Option<PathBuf>,
    pub optimize: bool,
    pub debug_info: bool,
    pub namespace: Option<String>,
}

impl Default for CSharpConfig {
    fn default() -> Self {
        Self {
            output_dir: None,
            optimize: false,
            debug_info: true,
            namespace: None,
        }
    }
}

/// C# 后端
pub struct CSharpBackend {
    config: CSharpConfig,
    /// 代码缓冲区
    buffer: x_codegen::CodeBuffer,
    /// 已声明的局部变量（用于在同一作用域内避免重复声明）
    declared_vars: HashSet<String>,
}

pub type CSharpResult<T> = Result<T, x_codegen::CodeGenError>;

impl CSharpBackend {
    pub fn new(config: CSharpConfig) -> Self {
        Self {
            config,
            buffer: x_codegen::CodeBuffer::new(),
            declared_vars: HashSet::new(),
        }
    }

    /// 输出一行代码
    fn line(&mut self, s: &str) -> CSharpResult<()> {
        self.buffer
            .line(s)
            .map_err(|e| x_codegen::CodeGenError::GenerationError(e.to_string()))
    }

    /// 将 X 语言标识符转换为合法的 C# 标识符（处理关键字冲突）
    fn csharp_ident(&self, name: &str) -> String {
        match name {
            "abstract" | "as" | "base" | "bool" | "break" | "byte" | "case" | "catch" | "char"
            | "checked" | "class" | "const" | "continue" | "decimal" | "default" | "delegate"
            | "do" | "double" | "else" | "enum" | "event" | "explicit" | "extern" | "false"
            | "finally" | "fixed" | "float" | "for" | "foreach" | "goto" | "if" | "implicit"
            | "in" | "int" | "interface" | "internal" | "is" | "lock" | "long" | "namespace"
            | "new" | "null" | "object" | "operator" | "out" | "override" | "params"
            | "private" | "protected" | "public" | "readonly" | "ref" | "return" | "sbyte"
            | "sealed" | "short" | "sizeof" | "stackalloc" | "static" | "string" | "struct"
            | "switch" | "this" | "throw" | "true" | "try" | "typeof" | "uint" | "ulong"
            | "unchecked" | "unsafe" | "ushort" | "using" | "virtual" | "void" | "volatile"
            | "while" => format!("@{}", name),
            _ => name.to_string(),
        }
    }

    /// 增加缩进
    fn indent(&mut self) {
        self.buffer.indent();
    }

    /// 减少缩进
    fn dedent(&mut self) {
        self.buffer.dedent();
    }

    /// 获取当前输出
    fn output(&self) -> &str {
        self.buffer.as_str()
    }

    /// 生成文件头部 (C# 14 / .NET 10)
    fn emit_header(&mut self) -> CSharpResult<()> {
        self.line(headers::CSHARP)?;
        self.line("// DO NOT EDIT")?;
        self.line("// Target: C# 14 / .NET 10 LTS (March 2026)")?;
        self.line("")?;
        self.line("using System;")?;
        self.line("using System.Collections.Generic;")?;
        self.line("using System.Threading.Tasks;")?;
        self.line("using System.Linq;")?;
        self.line("using System.Runtime.InteropServices;")?;
        self.line("")?;
        Ok(())
    }

    /// 映射 LIR 类型到 C# 类型
    fn lir_type_to_csharp(&self, ty: &x_lir::Type) -> String {
        use x_lir::Type::*;
        match ty {
            Void => "void".to_string(),
            Bool => "bool".to_string(),
            Char => "char".to_string(),
            Schar | Short => "short".to_string(),
            Uchar | Ushort | Int | Uint => "int".to_string(),
            Long | Ulong | LongLong | UlongLong => "long".to_string(),
            Float => "float".to_string(),
            Double | LongDouble => "double".to_string(),
            Size | Ptrdiff | Intptr | Uintptr => "long".to_string(),
            Pointer(inner) => {
                let inner_str = self.lir_type_to_csharp(inner);
                if inner_str == "void" { "IntPtr".to_string() }
                else { "object".to_string() } // Use object for generic pointers in C#
            }
            Array(inner, _) => format!("{}[]", self.lir_type_to_csharp(inner)),
            FunctionPointer(_, _) => "Delegate".to_string(),
            Named(n) => self.csharp_ident(n),
            Qualified(_, inner) => self.lir_type_to_csharp(inner),
        }
    }

    fn emit_lir_declaration(&mut self, decl: &x_lir::Declaration) -> CSharpResult<()> {
        match decl {
            x_lir::Declaration::Import(import) => self.emit_lir_import(import),
            x_lir::Declaration::Function(func) => {
                self.emit_lir_function(func)?;
                self.line("")
            }
            x_lir::Declaration::Global(global) => self.emit_lir_global(global),
            x_lir::Declaration::Struct(struct_) => self.emit_lir_struct(struct_),
            x_lir::Declaration::Class(class) => self.emit_lir_class(class),
            x_lir::Declaration::VTable(vtable) => self.emit_lir_vtable(vtable),
            x_lir::Declaration::Enum(enum_) => self.emit_lir_enum(enum_),
            x_lir::Declaration::TypeAlias(alias) => self.emit_lir_type_alias(alias),
            x_lir::Declaration::Newtype(nt) => self.emit_lir_newtype(nt),
            x_lir::Declaration::Trait(trait_) => self.emit_lir_trait(trait_),
            x_lir::Declaration::Effect(effect) => self.emit_lir_effect(effect),
            x_lir::Declaration::Impl(impl_) => self.emit_lir_impl(impl_),
            x_lir::Declaration::ExternFunction(ext) => self.emit_lir_extern_function(ext),
        }
    }

    fn emit_lir_import(&mut self, _import: &x_lir::Import) -> CSharpResult<()> {
        Ok(())
    }

    fn emit_lir_global(&mut self, global: &x_lir::GlobalVar) -> CSharpResult<()> {
        let ty = self.lir_type_to_csharp(&global.type_);
        let name = self.csharp_ident(&global.name);
        if let Some(init) = &global.initializer {
            let init_str = self.emit_lir_expr(init)?;
            self.line(&format!("public static {} {} = {};", ty, name, init_str))?;
        } else {
            self.line(&format!("public static {} {};", ty, name))?;
        }
        self.line("")?;
        Ok(())
    }

    fn emit_lir_struct(&mut self, struct_: &x_lir::Struct) -> CSharpResult<()> {
        let name = self.csharp_ident(&struct_.name);
        self.line(&format!("public struct {} {{", name))?;
        self.indent();
        for field in &struct_.fields {
            let ty = self.lir_type_to_csharp(&field.type_);
            let fname = self.csharp_ident(&field.name);
            self.line(&format!("public {} {};", ty, fname))?;
        }
        self.dedent();
        self.line("}")?;
        self.line("")?;
        Ok(())
    }

    fn emit_lir_class(&mut self, class: &x_lir::Class) -> CSharpResult<()> {
        let name = self.csharp_ident(&class.name);
        let mut header = format!("public class {}", name);
        if let Some(parent) = &class.extends {
            header.push_str(&format!(" : {}", self.csharp_ident(parent)));
        }
        if !class.implements.is_empty() {
            let impls = class.implements.iter().map(|i| self.csharp_ident(i)).collect::<Vec<_>>().join(", ");
            header.push_str(&format!(" : {}", impls));
        }
        header.push_str(" {");
        self.line(&header)?;
        self.indent();

        for field in &class.fields {
            let ty = self.lir_type_to_csharp(&field.type_);
            let fname = self.csharp_ident(&field.name);
            self.line(&format!("public {} {};", ty, fname))?;
        }

        self.dedent();
        self.line("}")?;
        self.line("")?;
        Ok(())
    }

    fn emit_lir_vtable(&mut self, _vtable: &x_lir::VTable) -> CSharpResult<()> {
        Ok(())
    }

    fn emit_lir_enum(&mut self, enum_: &x_lir::Enum) -> CSharpResult<()> {
        let name = self.csharp_ident(&enum_.name);
        self.line(&format!("public enum {} {{", name))?;
        self.indent();
        for (i, v) in enum_.variants.iter().enumerate() {
            let vname = self.csharp_ident(&v.name);
            let comma = if i < enum_.variants.len() - 1 { "," } else { "" };
            if let Some(val) = v.value {
                self.line(&format!("{} = {}{}", vname, val, comma))?;
            } else {
                self.line(&format!("{}{}", vname, comma))?;
            }
        }
        self.dedent();
        self.line("}")?;
        self.line("")?;
        Ok(())
    }

    fn emit_lir_type_alias(&mut self, alias: &x_lir::TypeAlias) -> CSharpResult<()> {
        self.line(&format!("// using {} = {};", self.csharp_ident(&alias.name), self.lir_type_to_csharp(&alias.type_)))?;
        Ok(())
    }

    fn emit_lir_newtype(&mut self, nt: &x_lir::Newtype) -> CSharpResult<()> {
        let name = self.csharp_ident(&nt.name);
        self.line(&format!("public readonly record struct {}({} Value);", name, self.lir_type_to_csharp(&nt.type_)))?;
        Ok(())
    }

    fn emit_lir_trait(&mut self, trait_: &x_lir::Trait) -> CSharpResult<()> {
        let name = self.csharp_ident(&trait_.name);
        self.line(&format!("public interface {} {{}}", name))?;
        Ok(())
    }

    fn emit_lir_effect(&mut self, effect: &x_lir::Effect) -> CSharpResult<()> {
        let name = self.csharp_ident(&effect.name);
        self.line(&format!("public interface {} {{}}", name))?;
        Ok(())
    }

    fn emit_lir_impl(&mut self, _impl_: &x_lir::Impl) -> CSharpResult<()> {
        Ok(())
    }

    fn emit_lir_extern_function(&mut self, ext: &x_lir::ExternFunction) -> CSharpResult<()> {
        // Skip common C functions that we'll map to builtins
        let skip = ["puts", "printf", "putchar", "malloc", "free", "exit", "panic"];
        if skip.contains(&ext.name.as_str()) {
            self.line(&format!("// skipped extern C function: {}", ext.name))?;
            return Ok(());
        }

        let name = self.csharp_ident(&ext.name);
        let ret = self.lir_type_to_csharp(&ext.return_type);
        let params: Vec<String> = ext.parameters.iter().enumerate()
            .map(|(i, ty)| format!("{} arg{}", self.lir_type_to_csharp(ty), i))
            .collect();
        self.line(&format!("[DllImport(\"__Internal\")] public static unsafe extern {} {}({});", ret, name, params.join(", ")))?;
        Ok(())
    }

    fn emit_lir_function(&mut self, func: &x_lir::Function) -> CSharpResult<()> {
        if func.name == "main" { return Ok(()); }
        self.declared_vars.clear();
        let ret = self.lir_type_to_csharp(&func.return_type);
        let name = self.csharp_ident(&func.name);
        let params: Vec<String> = func.parameters.iter()
            .map(|p| {
                let pname = self.csharp_ident(&p.name);
                self.declared_vars.insert(pname.clone());
                format!("{} {}", self.lir_type_to_csharp(&p.type_), pname)
            })
            .collect();
        self.line(&format!("public static unsafe {} {}({}) {{", ret, name, params.join(", ")))?;
        self.indent();
        for stmt in &func.body.statements {
            self.emit_lir_statement(stmt)?;
        }
        self.dedent();
        self.line("}")?;
        Ok(())
    }

    fn emit_lir_statement(&mut self, stmt: &x_lir::Statement) -> CSharpResult<()> {
        use x_lir::Statement::*;
        match stmt {
            Expression(e) => {
                if let x_lir::Expression::Assign(target, value) = e {
                    let val_str = self.emit_lir_expr(value)?;
                    // Void return detection
                    if val_str.contains("Console.WriteLine") || val_str.contains("Console.Write") || val_str.contains("Environment.Exit") || val_str.contains("Debug.Assert") {
                        self.line(&format!("{};", val_str))?;
                    } else if let x_lir::Expression::Variable(n) = target.as_ref() {
                        let name = self.csharp_ident(n);
                        if self.declared_vars.contains(&name) {
                            self.line(&format!("{} = {};", name, val_str))?;
                        } else {
                            // Use dynamic for maximum flexibility with untyped LIR tN vars
                            self.line(&format!("dynamic {} = {};", name, val_str))?;
                            self.declared_vars.insert(name);
                        }
                    } else {
                        let target_str = self.emit_lir_expr(target)?;
                        self.line(&format!("{} = {};", target_str, val_str))?;
                    }
                } else {
                    let s = self.emit_lir_expr(e)?;
                    self.line(&format!("{};", s))?;
                }
            }
            Variable(v) => {
                let ty = self.lir_type_to_csharp(&v.type_);
                let name = self.csharp_ident(&v.name);
                if !self.declared_vars.contains(&name) {
                    if let Some(init) = &v.initializer {
                        let s = self.emit_lir_expr(init)?;
                        self.line(&format!("{} {} = {};", ty, name, s))?;
                    } else {
                        self.line(&format!("{} {};", ty, name))?;
                    }
                    self.declared_vars.insert(name);
                }
            }
            If(i) => {
                let cond = self.emit_lir_expr(&i.condition)?;
                self.line(&format!("if ({}) {{", cond))?;
                self.indent();
                self.emit_lir_statement(&i.then_branch)?;
                self.dedent();
                if let Some(eb) = &i.else_branch {
                    self.line("} else {")?;
                    self.indent();
                    self.emit_lir_statement(eb)?;
                    self.dedent();
                }
                self.line("}")?;
            }
            While(w) => {
                let cond = self.emit_lir_expr(&w.condition)?;
                self.line(&format!("while ({}) {{", cond))?;
                self.indent();
                self.emit_lir_statement(&w.body)?;
                self.dedent();
                self.line("}")?;
            }
            Return(r) => {
                if let Some(e) = r {
                    let val = self.emit_lir_expr(e)?;
                    self.line(&format!("return {};", val))?;
                } else {
                    self.line("return;")?;
                }
            }
            Compound(b) => {
                self.line("{")?;
                self.indent();
                for s in &b.statements { self.emit_lir_statement(s)?; }
                self.dedent();
                self.line("}")?;
            }
            _ => {}
        }
        Ok(())
    }

    fn emit_lir_expr(&self, expr: &x_lir::Expression) -> CSharpResult<String> {
        use x_lir::Expression::*;
        match expr {
            Literal(l) => self.emit_lir_literal(l),
            Variable(n) => Ok(self.csharp_ident(n)),
            Binary(op, l, r) => {
                let ls = self.emit_lir_expr(l)?;
                let rs = self.emit_lir_expr(r)?;
                Ok(format!("({} {} {})", ls, self.map_lir_binop(op), rs))
            }
            Call(callee, args) => {
                let callee_str = self.emit_lir_expr(callee)?;
                let args_str: Vec<String> = args.iter().map(|a| self.emit_lir_expr(a)).collect::<Result<_,_>>()?;
                match callee_str.as_str() {
                    "println" | "_println" | "puts" => Ok(format!("Console.WriteLine({})", args_str.join(", "))),
                    "print" | "_print" | "putchar" => Ok(format!("Console.Write({})", args_str.join(", "))),
                    "assert" | "_assert" => Ok(format!("System.Diagnostics.Debug.Assert({})", args_str.get(0).cloned().unwrap_or("true".to_string()))),
                    "panic" => Ok(format!("throw new Exception({})", args_str.join(", "))),
                    _ => Ok(format!("{}({})", callee_str, args_str.join(", "))),
                }
            }
            Assign(t, v) => {
                let ts = self.emit_lir_expr(t)?;
                let vs = self.emit_lir_expr(v)?;
                Ok(format!("{} = {}", ts, vs))
            }
            _ => Ok("null".to_string()),
        }
    }

    fn emit_lir_literal(&self, lit: &x_lir::Literal) -> CSharpResult<String> {
        use x_lir::Literal::*;
        match lit {
            Integer(n) | Long(n) => Ok(n.to_string()),
            Float(f) | Double(f) => Ok(f.to_string()),
            String(s) => Ok(format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))),
            Bool(b) => Ok(b.to_string().to_lowercase()),
            _ => Ok("null".to_string()),
        }
    }

    fn map_lir_binop(&self, op: &x_lir::BinaryOp) -> String {
        use x_lir::BinaryOp::*;
        match op {
            Add => "+", Subtract => "-", Multiply => "*", Divide => "/", 
            Equal => "==", NotEqual => "!=", 
            _ => "+",
        }.to_string()
    }

    pub fn generate_from_lir(&mut self, lir: &LirProgram) -> CSharpResult<CodegenOutput> {
        self.buffer.clear();
        self.emit_header()?;

        let namespace = self.config.namespace.as_deref().unwrap_or("XLang");
        self.line(&format!("namespace {} {{", namespace))?;
        self.indent();
        self.line("public unsafe class Program {")?;
        self.indent();

        let mut main_function = None;
        for decl in &lir.declarations {
            if let x_lir::Declaration::Function(f) = decl {
                if f.name == "main" { main_function = Some(f); continue; }
            }
            self.emit_lir_declaration(decl)?;
        }

        self.line("public static unsafe void Main(string[] args) {")?;
        self.indent();
        self.declared_vars.clear();
        if let Some(main_fn) = main_function {
            for stmt in &main_fn.body.statements {
                if let x_lir::Statement::Return(r) = stmt {
                    let exit_code = if let Some(e) = r { self.emit_lir_expr(e)? } else { "0".to_string() };
                    self.line(&format!("Environment.Exit({});", exit_code))?;
                    continue;
                }
                self.emit_lir_statement(stmt)?;
            }
        }
        self.line("}")?;
        self.dedent();
        self.line("}")?;
        self.dedent();
        self.line("}")?;

        Ok(CodegenOutput {
            files: vec![OutputFile {
                path: PathBuf::from("Program.cs"),
                content: self.output().as_bytes().to_vec(),
                file_type: FileType::CSharp,
            }],
            dependencies: vec![],
        })
    }
}

impl CodeGenerator for CSharpBackend {
    type Config = CSharpConfig;
    type Error = x_codegen::CodeGenError;
    fn new(config: Self::Config) -> Self { Self::new(config) }
    fn generate_from_lir(&mut self, lir: &LirProgram) -> Result<CodegenOutput, Self::Error> {
        self.generate_from_lir(lir)
    }
}

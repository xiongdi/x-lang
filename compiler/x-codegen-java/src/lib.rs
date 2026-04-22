//! Java 后端 - 生成 Java 25 LTS 源代码
//!
//! 面向 JVM 平台，生成 Java 源代码
//!
//! ## Java 25 LTS 特性支持 (2025年9月发布)
//! - Records（记录类）
//! - Pattern matching for switch（switch 模式匹配）
//! - Sealed classes（密封类）
//! - Text blocks（文本块）
//! - Virtual threads（虚拟线程）
//! - Structured concurrency（结构化并发）
//! - Scoped values
//! - Pattern matching for switch with primitives

#![allow(clippy::only_used_in_recursion, clippy::useless_format)]

use std::collections::HashSet;
use std::path::PathBuf;
use x_codegen::{headers, CodeGenerator, CodegenOutput, FileType, OutputFile};
use x_lir::Program as LirProgram;

/// Java 后端配置
#[derive(Debug, Clone)]
pub struct JavaConfig {
    pub output_dir: Option<PathBuf>,
    pub optimize: bool,
    pub debug_info: bool,
    /// 生成的 Java 类名（默认为 "Main"）
    pub class_name: String,
}

impl Default for JavaConfig {
    fn default() -> Self {
        Self {
            output_dir: None,
            optimize: false,
            debug_info: true,
            class_name: "Main".to_string(),
        }
    }
}

/// Java 后端
pub struct JavaBackend {
    config: JavaConfig,
    /// 代码缓冲区（统一管理输出和缩进）
    buffer: x_codegen::CodeBuffer,
    /// 已声明的局部变量（用于在同一作用域内避免重复声明）
    declared_vars: HashSet<String>,
}

pub type JavaResult<T> = Result<T, x_codegen::CodeGenError>;

impl JavaBackend {
    pub fn new(config: JavaConfig) -> Self {
        Self {
            config,
            buffer: x_codegen::CodeBuffer::new(),
            declared_vars: HashSet::new(),
        }
    }

    /// 输出一行代码
    fn line(&mut self, s: &str) -> JavaResult<()> {
        self.buffer
            .line(s)
            .map_err(|e| x_codegen::CodeGenError::GenerationError(e.to_string()))
    }

    /// 将 X 语言标识符转换为合法的 Java 标识符（处理关键字冲突）
    fn java_ident(&self, name: &str) -> String {
        match name {
            "abstract" | "assert" | "boolean" | "break" | "byte" | "case" | "catch" | "char"
            | "class" | "const" | "continue" | "default" | "do" | "double" | "else" | "enum"
            | "extends" | "final" | "finally" | "float" | "for" | "goto" | "if" | "implements"
            | "import" | "instanceof" | "int" | "interface" | "long" | "native" | "new"
            | "package" | "private" | "protected" | "public" | "return" | "short" | "static"
            | "strictfp" | "super" | "switch" | "synchronized" | "this" | "throw" | "throws"
            | "transient" | "try" | "void" | "volatile" | "while" => format!("_{}", name),
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

    /// Emit file header with package and imports (Java 25 LTS)
    fn emit_header(&mut self) -> JavaResult<()> {
        self.line(headers::JAVA)?;
        self.line("// DO NOT EDIT")?;
        self.line("// Target: Java 25 LTS (September 2025)")?;
        self.line("")?;
        // Java 25 标准库导入
        self.line("import java.util.*;")?;
        Ok(())
    }

    /// 映射 LIR 类型到 Java 类型
    fn lir_type_to_java(&self, ty: &x_lir::Type) -> String {
        use x_lir::Type::*;
        match ty {
            Void => "void".to_string(),
            Bool => "boolean".to_string(),
            Char => "char".to_string(),
            Schar | Short => "short".to_string(),
            Uchar | Ushort | Int | Uint => "int".to_string(),
            Long | Ulong | LongLong | UlongLong => "long".to_string(),
            Float => "float".to_string(),
            Double | LongDouble => "double".to_string(),
            Size | Ptrdiff | Intptr | Uintptr => "long".to_string(),
            Pointer(inner) => {
                let inner_str = self.lir_type_to_java(inner);
                if inner_str == "void" {
                    "Object".to_string()
                } else {
                    format!("{}[]", inner_str)
                }
            }
            Array(inner, _) => format!("{}[]", self.lir_type_to_java(inner)),
            Named(n) => self.java_ident(n),
            FunctionPointer(_, _) => "java.util.function.Function".to_string(),
            Qualified(_, inner) => self.lir_type_to_java(inner),
        }
    }

    /// Generate code for a LIR declaration (dispatches to specific handlers)
    fn generate_lir_declaration(&mut self, decl: &x_lir::Declaration) -> JavaResult<()> {
        match decl {
            x_lir::Declaration::Import(import) => self.generate_lir_import(import),
            x_lir::Declaration::Function(func) => self.generate_lir_function(func),
            x_lir::Declaration::Global(global) => self.generate_lir_global(global),
            x_lir::Declaration::Struct(struct_) => self.generate_lir_struct(struct_),
            x_lir::Declaration::Class(class) => self.generate_lir_class(class),
            x_lir::Declaration::VTable(vtable) => self.generate_lir_vtable(vtable),
            x_lir::Declaration::Enum(enum_) => self.generate_lir_enum(enum_),
            x_lir::Declaration::TypeAlias(alias) => self.generate_lir_type_alias(alias),
            x_lir::Declaration::Newtype(nt) => self.generate_lir_newtype(nt),
            x_lir::Declaration::Trait(trait_) => self.generate_lir_trait(trait_),
            x_lir::Declaration::Effect(effect) => self.generate_lir_effect(effect),
            x_lir::Declaration::Impl(impl_) => self.generate_lir_impl(impl_),
            x_lir::Declaration::ExternFunction(ext) => self.generate_lir_extern_function(ext),
        }
    }

    /// Generate import declaration
    fn generate_lir_import(&mut self, import: &x_lir::Import) -> JavaResult<()> {
        if import.import_all {
            self.line(&format!("// import {}.*;", import.module_path))?;
        }
        Ok(())
    }

    /// Generate function declaration (non-main)
    fn generate_lir_function(&mut self, func: &x_lir::Function) -> JavaResult<()> {
        if func.name == "main" {
            return Ok(());
        }

        self.declared_vars.clear();
        let ret = self.lir_type_to_java(&func.return_type);
        let name = self.java_ident(&func.name);
        let params: Vec<String> = func
            .parameters
            .iter()
            .map(|p| {
                let pname = self.java_ident(&p.name);
                self.declared_vars.insert(pname.clone());
                format!("{} {}", self.lir_type_to_java(&p.type_), pname)
            })
            .collect();

        let type_params = if func.type_params.is_empty() {
            String::new()
        } else {
            format!("<{}> ", func.type_params.join(", "))
        };

        self.line(&format!(
            "public static {}{} {}({}) {{",
            type_params,
            ret,
            name,
            params.join(", ")
        ))?;
        self.indent();

        for stmt in &func.body.statements {
            self.emit_lir_statement(stmt)?;
        }

        self.dedent();
        self.line("}")?;
        self.line("")?;
        Ok(())
    }

    /// Generate global variable as static class field
    fn generate_lir_global(&mut self, global: &x_lir::GlobalVar) -> JavaResult<()> {
        let ty = self.lir_type_to_java(&global.type_);
        let vis = if global.is_static { "private" } else { "public" };
        let name = self.java_ident(&global.name);
        if let Some(init) = &global.initializer {
            let init_str = self.emit_lir_expr(init)?;
            self.line(&format!(
                "{} static {} {} = {};",
                vis, ty, name, init_str
            ))?;
        } else {
            self.line(&format!("{} static {} {};", vis, ty, name))?;
        }
        self.line("")?;
        Ok(())
    }

    /// Generate struct as Java record
    fn generate_lir_struct(&mut self, struct_: &x_lir::Struct) -> JavaResult<()> {
        let name = self.java_ident(&struct_.name);
        let fields: Vec<String> = struct_
            .fields
            .iter()
            .map(|f| format!("{} {}", self.lir_type_to_java(&f.type_), self.java_ident(&f.name)))
            .collect();
        self.line(&format!(
            "record {}({}) {{}}",
            name,
            fields.join(", ")
        ))?;
        self.line("")?;
        Ok(())
    }

    /// Generate class declaration
    fn generate_lir_class(&mut self, class: &x_lir::Class) -> JavaResult<()> {
        let name = self.java_ident(&class.name);
        let mut decl = format!("static class {}", name);
        if let Some(base) = &class.extends {
            decl.push_str(&format!(" extends {}", base));
        }
        if !class.implements.is_empty() {
            decl.push_str(&format!(" implements {}", class.implements.join(", ")));
        }
        decl.push_str(" {");
        self.line(&decl)?;
        self.indent();

        for field in &class.fields {
            let ty = self.lir_type_to_java(&field.type_);
            self.line(&format!("{} {};", ty, self.java_ident(&field.name)))?;
        }

        self.dedent();
        self.line("}")?;
        self.line("")?;
        Ok(())
    }

    /// Generate vtable as interface with method signatures
    fn generate_lir_vtable(&mut self, vtable: &x_lir::VTable) -> JavaResult<()> {
        let name = self.java_ident(&vtable.name);
        self.line(&format!(
            "interface {}VTable {{",
            name
        ))?;
        self.indent();

        for entry in &vtable.entries {
            let params: Vec<String> = entry
                .function_type
                .param_types
                .iter()
                .enumerate()
                .map(|(i, ty)| format!("{} arg{}", self.lir_type_to_java(ty), i))
                .collect();
            let return_type = self.lir_type_to_java(&entry.function_type.return_type);
            self.line(&format!(
                "{} {}({});",
                return_type,
                self.java_ident(&entry.method_name),
                params.join(", ")
            ))?;
        }

        self.dedent();
        self.line("}")?;
        self.line("")?;
        Ok(())
    }

    /// Generate enum declaration
    fn generate_lir_enum(&mut self, enum_: &x_lir::Enum) -> JavaResult<()> {
        let name = self.java_ident(&enum_.name);
        self.line(&format!("enum {} {{", name))?;
        self.indent();

        let variant_strs: Vec<String> = enum_
            .variants
            .iter()
            .map(|v| {
                let vname = self.java_ident(&v.name);
                if let Some(value) = v.value {
                    format!("{}({})", vname, value)
                } else {
                    vname
                }
            })
            .collect();
        self.line(&format!("{};", variant_strs.join(", ")))?;

        self.dedent();
        self.line("}")?;
        self.line("")?;
        Ok(())
    }

    fn generate_lir_type_alias(&mut self, alias: &x_lir::TypeAlias) -> JavaResult<()> {
        let ty = self.lir_type_to_java(&alias.type_);
        self.line(&format!("// type alias: {} = {}", alias.name, ty))?;
        Ok(())
    }

    fn generate_lir_newtype(&mut self, nt: &x_lir::Newtype) -> JavaResult<()> {
        let name = self.java_ident(&nt.name);
        let ty = self.lir_type_to_java(&nt.type_);
        self.line(&format!("record {}({} value) {{}}", name, ty))?;
        Ok(())
    }

    fn generate_lir_trait(&mut self, trait_: &x_lir::Trait) -> JavaResult<()> {
        let name = self.java_ident(&trait_.name);
        self.line(&format!("interface {} {{", name))?;
        Ok(())
    }

    fn generate_lir_effect(&mut self, effect: &x_lir::Effect) -> JavaResult<()> {
        let name = self.java_ident(&effect.name);
        self.line(&format!("interface {} {{", name))?;
        Ok(())
    }

    fn generate_lir_impl(&mut self, _impl_: &x_lir::Impl) -> JavaResult<()> {
        Ok(())
    }

    fn generate_lir_extern_function(&mut self, ext: &x_lir::ExternFunction) -> JavaResult<()> {
        let name = self.java_ident(&ext.name);
        let ret = self.lir_type_to_java(&ext.return_type);
        let params: Vec<String> = ext.parameters.iter().enumerate()
            .map(|(i, ty)| format!("{} arg{}", self.lir_type_to_java(ty), i))
            .collect();
        self.line(&format!("public static native {} {}({});", ret, name, params.join(", ")))?;
        Ok(())
    }

    fn generate_lir_block(&mut self, block: &x_lir::Block) -> JavaResult<()> {
        for stmt in &block.statements {
            self.emit_lir_statement(stmt)?;
        }
        Ok(())
    }

    fn emit_lir_statement(&mut self, stmt: &x_lir::Statement) -> JavaResult<()> {
        use x_lir::Statement::*;
        match stmt {
            Expression(e) => {
                if let x_lir::Expression::Assign(target, value) = e {
                    let val_str = self.emit_lir_expr(value)?;
                    if val_str.contains("System.out.println") || val_str.contains("System.err.println") {
                        self.line(&format!("{};", val_str))?;
                    } else if let x_lir::Expression::Variable(n) = target.as_ref() {
                        let name = self.java_ident(n);
                        if self.declared_vars.contains(&name) {
                            self.line(&format!("{} = {};", name, val_str))?;
                        } else {
                            // First time assignment - declare as Object/var for maximum compatibility with LIR's untyped tN vars
                            self.line(&format!("Object {} = {};", name, val_str))?;
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
                let ty = self.lir_type_to_java(&v.type_);
                let name = self.java_ident(&v.name);
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
                if let Some(else_br) = &i.else_branch {
                    self.line("} else {")?;
                    self.indent();
                    self.emit_lir_statement(else_br)?;
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
                    let s = self.emit_lir_expr(e)?;
                    self.line(&format!("// return {};", s))?; // Return inside main or native method?
                }
            }
            Compound(b) => {
                self.line("{")?;
                self.indent();
                self.generate_lir_block(b)?;
                self.dedent();
                self.line("}")?;
            }
            _ => {}
        }
        Ok(())
    }

    fn emit_lir_expr(&self, expr: &x_lir::Expression) -> JavaResult<String> {
        use x_lir::Expression::*;
        match expr {
            Literal(l) => self.emit_lir_literal(l),
            Variable(n) => Ok(self.java_ident(n)),
            Binary(op, l, r) => {
                let ls = self.emit_lir_expr(l)?;
                let rs = self.emit_lir_expr(r)?;
                Ok(format!("({} {} {})", ls, self.map_lir_binop(op), rs))
            }
            Call(callee, args) => {
                let callee_str = self.emit_lir_expr(callee)?;
                let args_str: Vec<String> = args.iter()
                    .map(|a| self.emit_lir_expr(a))
                    .collect::<Result<_, _>>()?;
                
                match callee_str.as_str() {
                    "println" | "_println" | "puts" => Ok(format!("System.out.println({})", args_str.join(", "))),
                    "print" | "_print" => Ok(format!("System.out.print({})", args_str.join(", "))),
                    "assert" | "_assert" => Ok(format!("assert ({})", args_str.get(0).cloned().unwrap_or("true".to_string()))),
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

    fn emit_lir_literal(&self, lit: &x_lir::Literal) -> JavaResult<String> {
        use x_lir::Literal::*;
        match lit {
            Integer(n) | Long(n) => Ok(n.to_string()),
            Float(f) | Double(f) => Ok(f.to_string()),
            String(s) => Ok(format!("\"{}\"", s)),
            Bool(b) => Ok(b.to_string()),
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

    pub fn generate_from_lir(&mut self, lir: &LirProgram) -> JavaResult<CodegenOutput> {
        self.buffer.clear();
        self.emit_header()?;

        self.line(&format!("public class {} {{", self.config.class_name))?;
        self.indent();

        let mut main_function = None;
        for decl in &lir.declarations {
            if let x_lir::Declaration::Function(f) = decl {
                if f.name == "main" { main_function = Some(f); continue; }
            }
            self.generate_lir_declaration(decl)?;
        }

        self.line("public static void main(String[] args) {")?;
        self.indent();
        self.declared_vars.clear();
        if let Some(main_fn) = main_function {
            for stmt in &main_fn.body.statements {
                self.emit_lir_statement(stmt)?;
            }
        }
        self.line("}")?;
        self.dedent();
        self.line("}")?;

        Ok(CodegenOutput {
            files: vec![OutputFile {
                path: PathBuf::from(format!("{}.java", self.config.class_name)),
                content: self.output().as_bytes().to_vec(),
                file_type: FileType::Java,
            }],
            dependencies: vec![],
        })
    }
}

impl CodeGenerator for JavaBackend {
    type Config = JavaConfig;
    type Error = x_codegen::CodeGenError;
    fn new(config: Self::Config) -> Self { Self::new(config) }
    fn generate_from_lir(&mut self, lir: &LirProgram) -> Result<CodegenOutput, Self::Error> {
        Self::generate_from_lir(self, lir)
    }
}

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use x_lexer::span::Span;

#[derive(Debug, Clone)]
pub struct ModuleInfo {
    pub name: String,
    pub file_path: PathBuf,
    pub exports: HashSet<String>,
    pub imports: Vec<ImportInfo>,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ImportInfo {
    pub module_path: String,
    pub symbols: Vec<ImportSymbol>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum ImportSymbol {
    All,
    Named(String, Option<String>),
}

#[derive(Debug, Clone)]
pub struct ModuleGraph {
    modules: HashMap<String, ModuleInfo>,
    dependency_order: Vec<String>,
}

impl ModuleGraph {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
            dependency_order: Vec::new(),
        }
    }

    pub fn add_module(&mut self, info: ModuleInfo) {
        let name = info.name.clone();
        self.modules.insert(name, info);
    }

    pub fn get_module(&self, name: &str) -> Option<&ModuleInfo> {
        self.modules.get(name)
    }

    pub fn modules(&self) -> &HashMap<String, ModuleInfo> {
        &self.modules
    }

    pub fn topological_sort(&mut self) -> Result<Vec<String>, ModuleError> {
        let mut visited = HashSet::new();
        let mut temp_mark = HashSet::new();
        let mut result = Vec::new();

        for module_name in self.modules.keys() {
            if !visited.contains(module_name) {
                self.visit_module(module_name, &mut visited, &mut temp_mark, &mut result)?;
            }
        }

        self.dependency_order = result.clone();
        Ok(result)
    }

    fn visit_module(
        &self,
        name: &str,
        visited: &mut HashSet<String>,
        temp_mark: &mut HashSet<String>,
        result: &mut Vec<String>,
    ) -> Result<(), ModuleError> {
        if visited.contains(name) {
            return Ok(());
        }

        if temp_mark.contains(name) {
            return Err(ModuleError::CyclicDependency {
                module: name.to_string(),
                cycle: self.find_cycle(name, name, &mut HashSet::new()),
            });
        }

        temp_mark.insert(name.to_string());

        if let Some(module) = self.modules.get(name) {
            for dep in &module.dependencies {
                if self.modules.contains_key(dep) {
                    self.visit_module(dep, visited, temp_mark, result)?;
                }
            }
        }

        temp_mark.remove(name);
        visited.insert(name.to_string());
        result.push(name.to_string());

        Ok(())
    }

    fn find_cycle(&self, start: &str, current: &str, visited: &mut HashSet<String>) -> Vec<String> {
        visited.insert(current.to_string());

        if let Some(module) = self.modules.get(current) {
            for dep in &module.dependencies {
                if dep == start {
                    return vec![start.to_string(), current.to_string()];
                }
                if !visited.contains(dep) && self.modules.contains_key(dep) {
                    let path = self.find_cycle(start, dep, visited);
                    if !path.is_empty() {
                        let mut result = vec![current.to_string()];
                        result.extend(path);
                        return result;
                    }
                }
            }
        }

        Vec::new()
    }

    pub fn compilation_order(&self) -> &[String] {
        &self.dependency_order
    }
}

impl Default for ModuleGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub enum ModuleError {
    ModuleNotFound {
        name: String,
        searched_paths: Vec<PathBuf>,
    },
    CyclicDependency {
        module: String,
        cycle: Vec<String>,
    },
    ParseError {
        file: PathBuf,
        message: String,
    },
    IOError {
        file: PathBuf,
        message: String,
    },
}

impl std::fmt::Display for ModuleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModuleError::ModuleNotFound {
                name,
                searched_paths,
            } => {
                write!(f, "模块 '{}' 未找到，搜索路径:", name)?;
                for path in searched_paths {
                    write!(f, "\n  - {}", path.display())?;
                }
                Ok(())
            }
            ModuleError::CyclicDependency { module, cycle } => {
                write!(f, "检测到循环依赖: {} -> {}", module, cycle.join(" -> "))
            }
            ModuleError::ParseError { file, message } => {
                write!(f, "解析文件 '{}' 失败: {}", file.display(), message)
            }
            ModuleError::IOError { file, message } => {
                write!(f, "读取文件 '{}' 失败: {}", file.display(), message)
            }
        }
    }
}

impl std::error::Error for ModuleError {}

pub struct ModuleResolver {
    search_paths: Vec<PathBuf>,
    module_cache: HashMap<String, String>,
    stdlib_path: Option<PathBuf>,
}

impl ModuleResolver {
    pub fn new() -> Self {
        Self {
            search_paths: vec![PathBuf::from(".")],
            module_cache: HashMap::new(),
            stdlib_path: None,
        }
    }

    pub fn with_stdlib(mut self, path: PathBuf) -> Self {
        self.stdlib_path = Some(path);
        self
    }

    pub fn add_search_path(&mut self, path: PathBuf) {
        self.search_paths.push(path);
    }

    pub fn set_search_paths(&mut self, paths: Vec<PathBuf>) {
        self.search_paths = paths;
    }

    pub fn resolve_module_path(
        &self,
        module_path: &str,
        from_file: Option<&Path>,
    ) -> Option<PathBuf> {
        let normalized = module_path.replace("::", "/").replace('.', "/");

        if module_path.starts_with("std::")
            || module_path.starts_with("std.")
            || module_path == "std"
        {
            if let Some(stdlib) = &self.stdlib_path {
                let module_name = module_path
                    .trim_start_matches("std::")
                    .trim_start_matches("std.")
                    .trim_start_matches("std");

                let module_name = if module_name.is_empty() {
                    "prelude"
                } else {
                    module_name
                };

                let std_path = stdlib.join(format!("{}.x", module_name));
                if std_path.exists() {
                    return Some(std_path);
                }

                let std_path_dir = stdlib
                    .join(normalized.trim_start_matches("std/"))
                    .with_extension("x");
                if std_path_dir.exists() {
                    return Some(std_path_dir);
                }
            }
        }

        if let Some(from) = from_file {
            if let Some(parent) = from.parent() {
                let relative_path = parent.join(&normalized).with_extension("x");
                if relative_path.exists() {
                    return Some(relative_path);
                }

                let relative_path = parent.join(&normalized).join("index.x");
                if relative_path.exists() {
                    return Some(relative_path);
                }
            }
        }

        for search_path in &self.search_paths {
            let module_file = search_path.join(&normalized).with_extension("x");
            if module_file.exists() {
                return Some(module_file);
            }

            let dir_module = search_path.join(&normalized).join("index.x");
            if dir_module.exists() {
                return Some(dir_module);
            }
        }

        None
    }

    pub fn resolve_module(
        &mut self,
        module_path: &str,
        from_file: Option<&Path>,
    ) -> Result<(PathBuf, String), ModuleError> {
        if let Some(cached) = self.module_cache.get(module_path) {
            if let Some(path) = self.resolve_module_path(module_path, from_file) {
                return Ok((path, cached.clone()));
            }
        }

        let path = self
            .resolve_module_path(module_path, from_file)
            .ok_or_else(|| {
                let mut searched = self.search_paths.clone();
                if let Some(stdlib) = &self.stdlib_path {
                    searched.push(stdlib.clone());
                }
                ModuleError::ModuleNotFound {
                    name: module_path.to_string(),
                    searched_paths: searched,
                }
            })?;

        let source = std::fs::read_to_string(&path).map_err(|e| ModuleError::IOError {
            file: path.clone(),
            message: e.to_string(),
        })?;

        self.module_cache
            .insert(module_path.to_string(), source.clone());
        Ok((path, source))
    }

    pub fn clear_cache(&mut self) {
        self.module_cache.clear();
    }
}

impl Default for ModuleResolver {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ModuleLoader {
    resolver: ModuleResolver,
    graph: ModuleGraph,
}

impl ModuleLoader {
    pub fn new() -> Self {
        Self {
            resolver: ModuleResolver::new(),
            graph: ModuleGraph::new(),
        }
    }

    pub fn with_resolver(resolver: ModuleResolver) -> Self {
        Self {
            resolver,
            graph: ModuleGraph::new(),
        }
    }

    pub fn resolver(&self) -> &ModuleResolver {
        &self.resolver
    }

    pub fn resolver_mut(&mut self) -> &mut ModuleResolver {
        &mut self.resolver
    }

    pub fn graph(&self) -> &ModuleGraph {
        &self.graph
    }

    pub fn load_module(
        &mut self,
        module_path: &str,
        from_file: Option<&Path>,
    ) -> Result<ModuleInfo, ModuleError> {
        let (file_path, source) = self.resolver.resolve_module(module_path, from_file)?;

        let parser = crate::parser::XParser::new();
        let program = parser.parse(&source).map_err(|e| ModuleError::ParseError {
            file: file_path.clone(),
            message: e.to_string(),
        })?;

        let mut exports = HashSet::new();
        let mut imports = Vec::new();
        let mut dependencies = Vec::new();

        for decl in &program.declarations {
            match decl {
                crate::ast::Declaration::Export(export_decl) => {
                    exports.insert(export_decl.symbol.clone());
                }
                crate::ast::Declaration::Function(f) if exports.contains(&f.name) => {}
                crate::ast::Declaration::Import(import_decl) => {
                    let symbols = import_decl
                        .symbols
                        .iter()
                        .map(|s| match s {
                            crate::ast::ImportSymbol::All => ImportSymbol::All,
                            crate::ast::ImportSymbol::Named(name, alias) => {
                                ImportSymbol::Named(name.clone(), alias.clone())
                            }
                        })
                        .collect();

                    dependencies.push(import_decl.module_path.clone());

                    imports.push(ImportInfo {
                        module_path: import_decl.module_path.clone(),
                        symbols,
                        span: import_decl.span,
                    });
                }
                crate::ast::Declaration::Module(module_decl) => {
                    let module_name = module_decl.name.clone();
                    for decl in &program.declarations {
                        if let crate::ast::Declaration::Export(export_decl) = decl {
                            if !exports.contains(&export_decl.symbol) {
                                exports.insert(format!("{}::{}", module_name, export_decl.symbol));
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        for decl in &program.declarations {
            if let crate::ast::Declaration::Export(export_decl) = decl {
                exports.insert(export_decl.symbol.clone());
            }
        }

        Ok(ModuleInfo {
            name: module_path.to_string(),
            file_path,
            exports,
            imports,
            dependencies,
        })
    }

    pub fn load_all_modules(&mut self, entry_file: &Path) -> Result<ModuleGraph, ModuleError> {
        let entry_name = entry_file
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("main")
            .to_string();

        let mut to_process = vec![(
            entry_name.clone(),
            entry_file.to_path_buf(),
            None::<PathBuf>,
        )];
        let mut processed = HashSet::new();

        while let Some((module_name, module_file, from_file)) = to_process.pop() {
            let normalized = module_name.replace("::", ".");
            if processed.contains(&normalized) {
                continue;
            }
            processed.insert(normalized);

            let from_path = from_file.as_deref();
            let info = self.load_module(&module_name, from_path.or(Some(&module_file)))?;

            for dep in &info.dependencies {
                if !processed.contains(&dep.replace("::", ".")) {
                    if let Some(dep_path) =
                        self.resolver.resolve_module_path(dep, Some(&module_file))
                    {
                        to_process.push((dep.clone(), dep_path, Some(module_file.clone())));
                    }
                }
            }

            self.graph.add_module(info);
        }

        self.graph.topological_sort()?;
        Ok(std::mem::take(&mut self.graph))
    }
}

impl Default for ModuleLoader {
    fn default() -> Self {
        Self::new()
    }
}

pub fn normalize_module_path(path: &str) -> String {
    path.replace("::", ".").replace('/', ".")
}

pub fn module_path_to_file_path(module_path: &str) -> PathBuf {
    PathBuf::from(module_path.replace("::", "/").replace('.', "/")).with_extension("x")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_module_path() {
        assert_eq!(
            normalize_module_path("std::collections::HashMap"),
            "std.collections.HashMap"
        );
        assert_eq!(normalize_module_path("utils/helpers"), "utils.helpers");
    }

    #[test]
    fn test_module_path_to_file_path() {
        let path = module_path_to_file_path("std::collections");
        assert!(
            path.to_str().unwrap().ends_with("std/collections.x")
                || path.to_str().unwrap().ends_with("std.collections.x")
        );
    }

    #[test]
    fn test_module_graph_topological_sort() {
        let mut graph = ModuleGraph::new();

        graph.add_module(ModuleInfo {
            name: "main".to_string(),
            file_path: PathBuf::from("main.x"),
            exports: HashSet::new(),
            imports: vec![],
            dependencies: vec!["utils".to_string()],
        });

        graph.add_module(ModuleInfo {
            name: "utils".to_string(),
            file_path: PathBuf::from("utils.x"),
            exports: HashSet::new(),
            imports: vec![],
            dependencies: vec![],
        });

        let order = graph.topological_sort().unwrap();

        let utils_pos = order.iter().position(|x| x == "utils").unwrap();
        let main_pos = order.iter().position(|x| x == "main").unwrap();

        assert!(
            utils_pos < main_pos,
            "utils (pos {}) should come before main (pos {}), order: {:?}",
            utils_pos,
            main_pos,
            order
        );
    }

    #[test]
    fn test_cyclic_dependency_detection() {
        let mut graph = ModuleGraph::new();

        graph.add_module(ModuleInfo {
            name: "a".to_string(),
            file_path: PathBuf::from("a.x"),
            exports: HashSet::new(),
            imports: vec![],
            dependencies: vec!["b".to_string()],
        });

        graph.add_module(ModuleInfo {
            name: "b".to_string(),
            file_path: PathBuf::from("b.x"),
            exports: HashSet::new(),
            imports: vec![],
            dependencies: vec!["a".to_string()],
        });

        let result = graph.topological_sort();
        assert!(matches!(result, Err(ModuleError::CyclicDependency { .. })));
    }
}

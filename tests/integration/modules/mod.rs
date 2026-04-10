mod module_tests;

#[test]
fn test_simple_module_import() {
    use std::path::PathBuf;
    
    let test_dir = PathBuf::from("tests/integration/modules/simple_module");
    let main_file = test_dir.join("main.x");
    
    assert!(main_file.exists(), "Test file {:?} does not exist", main_file);
    
    let content = std::fs::read_to_string(&main_file)
        .expect("Failed to read main.x");
    
    assert!(content.contains("import helper"), "main.x should import helper");
    assert!(content.contains("helper()"), "main.x should call helper function");
}

#[test]
fn test_nested_imports() {
    use std::path::PathBuf;
    
    let test_dir = PathBuf::from("tests/integration/modules/nested_imports");
    let main_file = test_dir.join("main.x");
    let middle_file = test_dir.join("middle.x");
    let base_file = test_dir.join("base.x");
    
    assert!(main_file.exists());
    assert!(middle_file.exists());
    assert!(base_file.exists());
    
    let main_content = std::fs::read_to_string(&main_file).unwrap();
    let middle_content = std::fs::read_to_string(&middle_file).unwrap();
    let base_content = std::fs::read_to_string(&base_file).unwrap();
    
    assert!(main_content.contains("import middle"));
    assert!(middle_content.contains("import base"));
    assert!(base_content.contains("export base_value"));
}

#[test]
fn test_cyclic_dependency_files() {
    use std::path::PathBuf;
    
    let test_dir = PathBuf::from("tests/integration/modules/cyclic_deps");
    let a_file = test_dir.join("a.x");
    let b_file = test_dir.join("b.x");
    
    assert!(a_file.exists());
    assert!(b_file.exists());
    
    let a_content = std::fs::read_to_string(&a_file).unwrap();
    let b_content = std::fs::read_to_string(&b_file).unwrap();
    
    assert!(a_content.contains("import b"));
    assert!(b_content.contains("import a"));
}

#[test]
fn test_module_graph_topological_sort() {
    use x_parser::module_resolver::{ModuleGraph, ModuleInfo};
    use std::collections::HashSet;
    use std::path::PathBuf;
    
    let mut graph = ModuleGraph::new();
    
    graph.add_module(ModuleInfo {
        name: "main".to_string(),
        file_path: PathBuf::from("main.x"),
        exports: HashSet::new(),
        imports: vec![],
        dependencies: vec!["utils".to_string(), "types".to_string()],
    });
    
    graph.add_module(ModuleInfo {
        name: "utils".to_string(),
        file_path: PathBuf::from("utils.x"),
        exports: HashSet::new(),
        imports: vec![],
        dependencies: vec![],
    });
    
    graph.add_module(ModuleInfo {
        name: "types".to_string(),
        file_path: PathBuf::from("types.x"),
        exports: HashSet::new(),
        imports: vec![],
        dependencies: vec!["utils".to_string()],
    });
    
    let order = graph.topological_sort().expect("Topological sort should succeed");
    
    assert!(order.contains(&"utils".to_string()));
    assert!(order.contains(&"types".to_string()));
    assert!(order.contains(&"main".to_string()));
    
    let utils_pos = order.iter().position(|x| x == "utils").unwrap();
    let types_pos = order.iter().position(|x| x == "types").unwrap();
    let main_pos = order.iter().position(|x| x == "main").unwrap();
    
    assert!(utils_pos < types_pos, "utils should come before types");
    assert!(types_pos < main_pos, "types should come before main");
}

#[test]
fn test_cyclic_dependency_detection() {
    use x_parser::module_resolver::{ModuleGraph, ModuleInfo, ModuleError};
    use std::collections::HashSet;
    use std::path::PathBuf;
    
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

#[test]
fn test_module_path_normalization() {
    use x_parser::module_resolver::normalize_module_path;
    
    assert_eq!(normalize_module_path("std::collections::HashMap"), "std.collections.HashMap");
    assert_eq!(normalize_module_path("utils/helpers"), "utils.helpers");
    assert_eq!(normalize_module_path("simple"), "simple");
}

#[test]
fn test_module_examples_exist() {
    use std::path::PathBuf;
    
    let examples_dir = PathBuf::from("examples/modules");
    
    assert!(examples_dir.join("main.x").exists());
    assert!(examples_dir.join("utils.x").exists());
    assert!(examples_dir.join("types.x").exists());
    assert!(examples_dir.join("helpers/math.x").exists());
    assert!(examples_dir.join("helpers/string.x").exists());
}

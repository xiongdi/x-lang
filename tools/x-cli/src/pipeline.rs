pub fn run_pipeline(
    source: &str,
) -> Result<
    (
        x_parser::ast::Program,
        x_hir::Hir,
        x_perceus::PerceusIR,
    ),
    String,
> {
    let parser = x_parser::parser::XParser::new();
    let program = parser.parse(source).map_err(|e| format!("解析错误: {}", e))?;

    x_typechecker::type_check(&program).map_err(|e| format!("类型检查错误: {}", e))?;

    let hir = x_hir::ast_to_hir(&program).map_err(|e| format!("HIR 转换错误: {}", e))?;

    let pir =
        x_perceus::analyze_hir(&hir).map_err(|e| format!("Perceus 分析错误: {}", e))?;

    Ok((program, hir, pir))
}

pub fn format_parse_error(
    file: &str,
    source: &str,
    e: &x_parser::errors::ParseError,
) -> String {
    if let Some(span) = e.span() {
        let (line, col) = span.line_col(source);
        let snippet = span.snippet(source);
        format!(
            "{}:{}:{}: {}\n  {} | {}",
            file,
            line,
            col,
            e,
            line,
            snippet.trim_end()
        )
    } else {
        format!("{}: {}", file, e)
    }
}


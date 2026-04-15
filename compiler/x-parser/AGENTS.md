# AGENTS.md — compiler/x-parser/

**AST generation via LALRPOP-based parser.** Converts token stream into typed AST.

---

## OVERVIEW

LALRPOP-based parser that transforms `x-lexer` tokens into a typed AST rooted in the `Program` node.
- **Input**: tokens from `compiler/x-lexer`
- **Output**: `Program` AST consumed by `compiler/x-typechecker`
- **Grammar**: LALRPOP file at `src/parser.lalrpop`
- **AST nodes**: Defined in `src/ast.rs` (Expr, Stmt, Program, etc.)

---

## INPUT/OUTPUT

| Direction | Source | Type | Notes |
|-----------|--------|------|-------|
| **Input** | `x-lexer` | Token stream | Via workspace dependency |
| **Output** | `x-typechecker` | `Program` AST | Root of typed tree |
| **AST root** | `src/ast.rs` | `Program` struct | All top-level definitions |

---

## LALRPOP USAGE

- **Grammar file**: `src/parser.lalrpop`
- **Build**: Automatic during `cargo build`; LALRPOP generates Rust code
- **To extend syntax**:
  1. Add rule to `parser.lalrpop`
  2. Add/update AST node in `src/ast.rs`
  3. Run `cargo test -p x-parser` to validate
  4. Fix type errors; re-test

- **Quick iteration**: `cargo test -p x-parser -- --nocapture` to see parse traces

---

## AST STRUCTURE

- **Root**: `Program` (module or file-level definitions)
- **Expression**: `Expr` (arithmetic, calls, literals, patterns, etc.)
- **Statement**: `Stmt` (assignments, conditionals, loops, etc.)
- **Common patterns**: Box for nested trees, Vec for sequences
- **Error variant**: `ParserError` for parse failures (Result<Program, ParserError>)

**Location**: `src/ast.rs` — edit here when adding new syntax constructs.

---

## COMMON TASKS

| Task | Steps |
|------|-------|
| **Add syntax** | Edit `parser.lalrpop`, add AST node in `ast.rs`, test |
| **Fix parse error** | Check LALRPOP grammar; validate AST node structure |
| **Optimize parsing** | Profile with `cargo test -p x-parser --release` |
| **Debug grammar conflict** | LALRPOP will report shift/reduce conflicts; refactor rules |

---

## DEPENDENCIES

- **Depends on**: `x-lexer` (tokens) via workspace
- **Depended on by**: `x-typechecker` (AST input)
- **Dependency chain**: lexer → **parser** → typechecker → hir → mir → lir

---

## CONVENTIONS

1. **Module layout**: `lib.rs` re-exports public types; grammar in `parser.lalrpop`
2. **Error handling**: `Result<Program, ParserError>` for parse entry point
3. **Testing**: Unit tests in crate; invalid input → parse error (not panic)
4. **Format**: Run `cargo fmt` after edits
5. **Type safety**: Leverage Rust's type system to prevent invalid AST states

---

## NEXT STEPS

- For **syntax design**, see root `AGENTS.md` → Feature Implementation Order (step 3 is "Update x-parser")
- For **type checking**, see `x-typechecker/`
- For **integration**, see `tools/x-cli/AGENTS.md`

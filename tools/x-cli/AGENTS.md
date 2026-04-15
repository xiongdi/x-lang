# AGENTS.md — tools/x-cli/

**CLI entry point & pipeline orchestrator.** Exposes `run`, `check`, `compile` commands; drives compiler from lexer through codegen.

---

## OVERVIEW

Command-line interface for the X toolchain orchestrating the full compilation pipeline.
- **Binary name**: `x` (defined in `Cargo.toml` as `[[bin]]`)
- **Entry point**: `src/main.rs` (clap-based argument parser)
- **Pipeline driver**: `src/pipeline.rs` (sequences lexer → parser → typechecker → hir → mir → lir → codegen)
- **Output control**: `--emit` flag selects which IR stage to output

---

## COMMANDS

| Command | Effect |
|---------|--------|
| `x run <file.x>` | Parse → type-check → interpret (AST-based execution) |
| `x check <file.x>` | Parse → type-check only; report errors; no codegen |
| `x compile <file.x>` | Full pipeline → codegen → target code output |

---

## KEY FLAGS

| Flag | Purpose |
|------|---------|
| `--emit {tokens,ast,hir,mir,lir,zig,python,rust,...}` | Output IR stage or generated code |
| `-o, --output <path>` | Write result to file |
| `--backend {zig,python,rust,...}` | Select codegen backend (default: zig) |
| `--debug` | Increase logging verbosity |
| `--version` | Print version info |

---

## PIPELINE ORCHESTRATION

**File**: `src/pipeline.rs`

```
Input (file.x)
  ↓ [Lexer]
  ↓ [Parser]
  ↓ [Typechecker]
  ↓ [HIR] (optional --emit hir)
  ↓ [MIR] (optional --emit mir + Perceus marks)
  ↓ [LIR] (optional --emit lir)
  ↓ [Codegen: x-codegen-{backend}]
Output (target code or --emit stage)
```

**Emit stage mapping**: --emit flag gates output at each pipeline stage.

---

## COMMON TASKS

| Task | Steps |
|------|-------|
| **Add CLI flag** | 1. Add field to clap struct in main.rs; 2. Pass through pipeline.rs; 3. Test with `cargo run -- <cmd> --new-flag` |
| **Add --emit type** | 1. Add variant to `EmitStage` enum in pipeline.rs; 2. Extend match arms in emit logic; 3. Integrate codegen or IR output |
| **Change default backend** | Edit pipeline.rs backend selection (currently Zig) |
| **Test** | `cargo run -- run examples/hello.x` / `check` / `compile` |

---

## DEPENDENCIES

- **Upstream**: All compiler crates (x-lexer through x-codegen-*) via `../compiler/*` path dependencies
- **Parser**: Clap for CLI argument handling
- **No downstream**: CLI is final user-facing tool

---

## CONVENTIONS

1. **Deterministic behavior**: Same input → same output (crucial for reproducible builds)
2. **Clap struct**: Defines CLI interface; auto-generates `--help`
3. **Error propagation**: Use `Result<T, CliError>` through pipeline
4. **Logging**: Use `log::info!()` for major stages; `debug!()` for details
5. **Workspace deps**: Reference compiler crates via `path = "../compiler/..."` in Cargo.toml

---

## NEXT STEPS

- For **pipeline stages**, see `compiler/AGENTS.md`
- For **backend selection**, see `compiler/x-codegen/AGENTS.md`
- For **feature implementation**, see root `AGENTS.md` → Feature Implementation Order

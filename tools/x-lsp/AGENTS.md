# AGENTS.md — tools/x-lsp/

**Updated:** 2026-04-22 | **Tool:** Language Server

---

## OVERVIEW

Language Server - lsp-server + lsp-types, stdio communication with editors (VS Code, Neovim, etc).

---

## KEY MODULES

| Module | File | Purpose |
|--------|------|---------|
| `main` | src/main.rs | env_logger::init, LspServer::new()?.run() |
| `server` | src/server.rs | stdin/stdout message loop |
| `handlers` | src/handlers.rs | LSP requests (completion, diagnostics) |
| `analysis` | src/analysis.rs | Calls x-lexer/x-parser/x-typechecker |
| `state` | src/state.rs | Open documents, versions, diagnostics |

---

## CAPABILITIES

- [ ] Real-time diagnostics
- [ ] Code completion
- [ ] Hover information
- [ ] Go to definition
- [ ] Find references
- [ ] Document symbols
- [ ] Rename symbol
- [ ] Formatting

---

## DEPENDENCIES

- **Uses:** x-lexer, x-parser, x-typechecker
- **For:** Editor integration

---

## BUILD

```bash
cd tools/x-lsp && cargo build --release
```

---

## TESTS

```bash
cd tools && cargo test -p x-lsp
```

---

## NEXT STEPS

- See tools/CLAUDE.md for workspace context

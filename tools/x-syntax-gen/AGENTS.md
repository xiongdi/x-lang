# AGENTS.md — tools/x-syntax-gen/

**Updated:** 2026-04-22 | **Tool:** Syntax Generator

---

## OVERVIEW

Syntax highlighting asset generator - generates editor syntax definitions from x-lexer token model.

---

## WORKFLOW

1. `token_mapping::build_syntax_model()` - reads keywords from x-lexer
2. `generators::{vscode,vim,neovim,sublime,emacs,jetbrains}::generate`
3. Default output: `output/` directory

---

## SUBCOMMANDS

| Command | Output |
|---------|--------|
| `all` | All editor grammars |
| `vscode` | TextMate grammar |
| `vim` | Vim syntax |
| `neovim` | Neovim syntax |
| `sublime` | Sublime Text grammar |
| `emacs` | Emacs font-lock |
| `jetbrains` | JetBrains grammar |

---

## WHEN TO REGENERATE

- Modifying x-lexer/src/token.rs keywords
- Changing token categories
- Adding new token types

---

## DEPENDENCIES

- **Uses:** x-lexer (token definitions)
- **Generates:** Editor-specific syntax files

---

## TESTS

```bash
cd tools && cargo test -p x-syntax-gen
```

---

## NEXT STEPS

- See tools/CLAUDE.md for workspace context

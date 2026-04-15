# AGENTS.md — compiler/x-mir/

**Perceus reference counting analysis center.** MIR stage inserts dup/drop/reuse annotations for memory safety.

---

## OVERVIEW

Mid-level IR with **compile-time reference counting** (Perceus algorithm).
- **Input**: HIR from `x-hir` (semantic tree with type info)
- **Output**: MIR with dup/drop/reuse annotations for each value; feeds `x-lir`
- **Centerpiece**: Perceus analysis ensures zero-copy data updates while maintaining safety
- **Critical invariant**: Every value must have correct dup/drop marks (or compiler bug)

---

## RESPONSIBILITY

1. **Lower HIR → MIR**: Transform high-level IR into lower representation
2. **Insert dup marks**: Copy reference when value used in multiple places
3. **Insert drop marks**: Release reference when value goes out of scope
4. **Identify reuse opportunities**: When ref count = 1, mark for in-place mutation
5. **Validate safety**: Ensure all memory operations are sound

---

## PERCEUS ANALYSIS (Non-Negotiable)

| Term | Meaning | Safety Impact |
|------|---------|---------------|
| **dup** | Copy a reference (increment ref count) | Prevents use-after-free |
| **drop** | Release a reference (decrement ref count) | Ensures cleanup |
| **reuse** | In-place mutation when ref count = 1 | Eliminates allocation overhead |

**CRITICAL**: Any bug in dup/drop insertion = **memory safety violation** = **compiler bug**, not user error.

---

## KEY TYPES

- `MirModule`: Container for MIR functions and definitions
- `MirStatement`: Single statement with side effects
- `MirValue`: SSA-style value with dup/drop/reuse annotations
- `RefCount`: Tracks reference count state (shared, unique, etc.)
- `DupDropReuse`: Enum marking each value's handling (Dup | Drop | Reuse | Neither)

---

## COMMON TASKS

| Task | Approach |
|------|----------|
| **Debug dup/drop issue** | Enable trace logging in MIR stage; inspect annotations |
| **Add Perceus for new construct** | Extend `RefCount` analysis; add test case |
| **Test memory safety** | Run `cargo test -p x-mir --lib perceus` |
| **Validate reuse analysis** | Check that reuse marks align with ref count = 1 sites |

---

## DEPENDENCIES

- **Input**: `x-hir` (semantic tree) via workspace
- **Output**: `x-lir` consumes MIR with dup/drop marks
- **Coordination**: Perceus marks influence code generation in backend

---

## CRITICAL INVARIANTS

1. **Every value must have correct dup/drop marks** → incorrect marks = memory bug
2. **Reuse marks only appear when ref count = 1** → else allocation required
3. **drop always paired with corresponding dup** → ref count balanced
4. **Safety guaranteed if invariants hold** → no runtime checks needed
5. **Violations are compiler bugs** → users should never encounter unsafety

---

## CONVENTIONS

1. **Module layout**: `lib.rs` exposes public API; analysis logic in private modules
2. **Error handling**: `Result<MirModule, AnalysisError>` for lowering entry point
3. **Testing**: Dedicated memory safety tests (ref counting correctness, reuse detection)
4. **Logging**: Use `log::debug!()` to trace dup/drop insertion
5. **Determinism**: Same HIR input → same MIR output (for reproducible builds)

---

## NEXT STEPS

- For **Perceus theory**, see root `AGENTS.md` → "PERCEUS (Memory Management)"
- For **LIR lowering**, see `x-lir/`
- For **backend impact**, see `x-codegen/`
- For **testing**, see `tests/spec_runner/` (integration tests validate memory safety end-to-end)

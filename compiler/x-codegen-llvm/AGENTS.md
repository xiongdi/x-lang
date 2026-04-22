OVERVIEW: LLVM backend that translates x_lir into LLVM IR for deep optimization.

KEY TYPES:
- LlvmBackend: central backend implementation
- LlvmBackendConfig: configuration for codegen (target, opt level, debug flags)
- impl CodeGenerator for LlvmBackend: trait impl that generates LLVM IR from x_lir::Program

DEPENDENCIES:
- Input: x_lir::Program
- Output: LLVM IR string/module

TESTS:
- Run from repo root: cd compiler && cargo test -p x-codegen-llvm

KNOWLEDGE/BEHAVIORAL NOTES:
- The backend should preserve SSA form and maintain debug metadata when possible
- Lower level optimization passes are applied post IR emission
- Ensure module is valid LLVM IR before emitting to file or further stages

TYPES AND INTERFACES:
- LlvmBackend exposes a generate function taking a Program and returning IR
- LlvmBackendConfig may include optimization level, target triple, data layout
- The CodeGenerator trait requires a generate method and error handling

PROTOCOL:
- Input is consumed from x_lir
- Output is printed as LLVM IR string for downstream consumers
- Errors should be surfaced with descriptive messages

BUILD & TEST STRATEGY:
- Cargo feature flags can enable or disable LLVM specific passes
- Tests live under compiler/x-codegen-llvm/tests
- Integration tests validate IR validity and simple function emission

EXTENSIONS:
- Can be adapted to multiple backends by implementing CodeGenerator for each backend
- LLVM IR can be emitted to .ll files for inspection
- Support for target-specific attributes can be added later

RISKs:
- Mismatched data layout can cause undefined behavior in downstream passes
- Debug metadata may be stripped in release builds
- IR validation failures should fail the build early

NEXT ACTIONS:
- Implement LlvmBackend with minimal generate implementation
- Add unit tests asserting that a tiny x_lir::Program produces at least a valid IR snippet


<!-- OMO_INTERNAL_INITIATOR -->

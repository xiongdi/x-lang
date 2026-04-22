# AGENTS.md

Overview: Native/Wasm assembly backend - lowers from LIR to assembly text, then external assembler/linker. Generates x86_64, AArch64, RISC-V, Wasm.

KEY TYPES
- NativeBackend + NativeBackendConfig
  - TargetArch: X86_64, AArch64, RISC-V, Wasm
  - TargetOS: Linux, Windows, macOS, Wasm
  - OutputFormat: AsmText, Object, Binary, WasmText
- impl CodeGenerator for NativeBackend
  - generate_from_lir: main entry point

DEPENDENCIES
- Input: LIR from x-lir
- Module layout:
  - arch/
  - assembly/
  - assembler/
  - encoding/

FLOW
- Receive LIR, select path by TargetArch
- Emit assembly into assembly/ per function
- Emit metadata into arch/ (prologs, epilogs)
- Use assembler/encoding for instruction encoding hints
- Finalize to object or wasm text via OutputFormat

TESTS
- cd compiler && cargo test -p x-codegen-asm
- Validate that emitted assembly compiles with external toolchain

CONFIG CHOICES
- X86_64: platform specifics handled in arch/x86_64/
- AArch64: NEON features toggles
- RISC-V: RV64GC features, float support
- Wasm: Wasm text emission as fallback

BACKEND INTERFACE
- Trait: CodeGenerator
- Required methods: generate_from_lir(&self, lir: &Lir) -> Result<String, Error>
- Output is emitted in a deterministic order
- Errors map to compile failures with clear messages

OPTIMIZATIONS
- Basic peephole like optimizations in assembly
- Inline small constants; avoid spills

MAINTENANCE NOTES
- Follow PROJECT CODING STANDARDS
- Keep formatting ASCII friendly
- Add tests for new archs in separate PR

<!-- OMO_INTERNAL_INITIATOR -->

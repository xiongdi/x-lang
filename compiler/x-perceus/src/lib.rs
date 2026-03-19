//! Legacy compatibility facade for Perceus.
//!
//! The canonical Perceus implementation now lives in `x-mir` as part of the
//! middle-end pipeline:
//!
//! HIR -> MIR (+ Perceus) -> LIR
//!
//! This crate is retained only for backward compatibility so existing code that
//! imports `x_perceus::*` continues to compile during the architecture
//! transition.

pub use x_mir::perceus::BasicBlock;
pub use x_mir::{
    analyze_hir, ControlFlowAnalysis, FunctionAnalysis, FunctionSignature, InterproceduralContext,
    MemoryOp, OwnershipFact, OwnershipState, ParamOwnershipBehavior, PerceusAnalyzer, PerceusError,
    PerceusIR, ReturnOwnershipBehavior, ReuseAnalysis, ReusePair, SourcePos,
};

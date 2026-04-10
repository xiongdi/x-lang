//! 数据流分析框架
//!
//! 该模块提供通用的数据流分析基础设施，包括：
//! - `DataflowAnalysis` trait：前向/后向分析框架
//! - `LivenessAnalysis`：活跃变量分析
//! - `ReachingDefinitions`：到达定义分析
//! - 可用表达式分析等

use crate::cfg::{Cfg, CfgError};
use crate::mir::*;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

pub type DataflowResult<T> = Result<T, CfgError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnalysisDirection {
    Forward,
    Backward,
}

pub trait DataflowDomain: Clone + Debug + PartialEq {
    fn top() -> Self;
    fn bottom() -> Self;
    fn meet(&self, other: &Self) -> Self;
    fn transfer(&self, instr: &MirInstruction) -> Self;
}

#[derive(Debug, Clone)]
pub struct DataflowAnalysisResult<D: DataflowDomain> {
    pub block_input: HashMap<MirBlockId, D>,
    pub block_output: HashMap<MirBlockId, D>,
}

pub trait DataflowAnalysis {
    type Domain: DataflowDomain;

    fn direction() -> AnalysisDirection;

    fn entry_value() -> Self::Domain;

    fn initial_value() -> Self::Domain;

    fn transfer(&self, domain: &Self::Domain, instr: &MirInstruction) -> Self::Domain;

    fn analyze(&self, cfg: &Cfg) -> DataflowResult<DataflowAnalysisResult<Self::Domain>> {
        let mut result = DataflowAnalysisResult {
            block_input: HashMap::new(),
            block_output: HashMap::new(),
        };

        let order = match Self::direction() {
            AnalysisDirection::Forward => cfg.reverse_postorder(),
            AnalysisDirection::Backward => {
                let mut order = cfg.depth_first_order();
                order.reverse();
                order
            }
        };

        for &block_id in &order {
            let input = if block_id == cfg.entry_block {
                Self::entry_value()
            } else {
                let preds = cfg.predecessors(block_id);
                if preds.is_empty() {
                    Self::initial_value()
                } else {
                    preds
                        .iter()
                        .filter_map(|p| result.block_output.get(p).cloned())
                        .fold(Self::Domain::top(), |acc, d| acc.meet(&d))
                }
            };

            result.block_input.insert(block_id, input.clone());

            let mut current = input;
            if let Some(block) = cfg.get_block(block_id) {
                for instr in &block.instructions {
                    current = self.transfer(&current, instr);
                }
            }

            result.block_output.insert(block_id, current);
        }

        Ok(result)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Definition {
    pub variable: MirLocalId,
    pub block_id: MirBlockId,
    pub instr_index: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReachingDefinitionsDomain {
    pub definitions: HashSet<Definition>,
}

impl DataflowDomain for ReachingDefinitionsDomain {
    fn top() -> Self {
        Self {
            definitions: HashSet::new(),
        }
    }

    fn bottom() -> Self {
        Self {
            definitions: HashSet::new(),
        }
    }

    fn meet(&self, other: &Self) -> Self {
        Self {
            definitions: self
                .definitions
                .union(&other.definitions)
                .cloned()
                .collect(),
        }
    }

    fn transfer(&self, _instr: &MirInstruction) -> Self {
        self.clone()
    }
}

#[derive(Debug, Clone)]
pub struct ReachingDefinitions {
    pub kill_map: HashMap<MirLocalId, HashSet<Definition>>,
}

impl ReachingDefinitions {
    pub fn new() -> Self {
        Self {
            kill_map: HashMap::new(),
        }
    }

    pub fn collect_definitions(cfg: &Cfg) -> HashMap<MirLocalId, HashSet<Definition>> {
        let mut def_map: HashMap<MirLocalId, HashSet<Definition>> = HashMap::new();

        for (&block_id, block) in &cfg.blocks {
            for (idx, instr) in block.instructions.iter().enumerate() {
                if let Some(def_var) = Self::get_defined_variable(instr) {
                    def_map.entry(def_var).or_default().insert(Definition {
                        variable: def_var,
                        block_id,
                        instr_index: idx,
                    });
                }
            }
        }

        def_map
    }

    fn get_defined_variable(instr: &MirInstruction) -> Option<MirLocalId> {
        match instr {
            MirInstruction::Assign { dest, .. }
            | MirInstruction::BinaryOp { dest, .. }
            | MirInstruction::UnaryOp { dest, .. }
            | MirInstruction::FieldAccess { dest, .. }
            | MirInstruction::ArrayAccess { dest, .. }
            | MirInstruction::Alloc { dest, .. }
            | MirInstruction::Load { dest, .. }
            | MirInstruction::Cast { dest, .. }
            | MirInstruction::Dup { dest, .. }
            | MirInstruction::Reuse { dest, .. } => Some(*dest),
            MirInstruction::Call { dest, .. } => dest.as_ref().copied(),
            MirInstruction::Store { .. } | MirInstruction::Drop { .. } => None,
        }
    }

    fn get_used_variables(instr: &MirInstruction) -> Vec<MirLocalId> {
        let mut vars = Vec::new();
        match instr {
            MirInstruction::Assign { value, .. } => {
                if let MirOperand::Local(id) = value {
                    vars.push(*id);
                }
            }
            MirInstruction::BinaryOp { left, right, .. } => {
                if let MirOperand::Local(id) = left {
                    vars.push(*id);
                }
                if let MirOperand::Local(id) = right {
                    vars.push(*id);
                }
            }
            MirInstruction::UnaryOp { operand, .. } => {
                if let MirOperand::Local(id) = operand {
                    vars.push(*id);
                }
            }
            MirInstruction::Call { args, .. } => {
                for arg in args {
                    if let MirOperand::Local(id) = arg {
                        vars.push(*id);
                    }
                }
            }
            MirInstruction::FieldAccess { object, .. }
            | MirInstruction::ArrayAccess { array: object, .. } => {
                if let MirOperand::Local(id) = object {
                    vars.push(*id);
                }
            }
            MirInstruction::Load { ptr, .. } | MirInstruction::Store { ptr, .. } => {
                if let MirOperand::Local(id) = ptr {
                    vars.push(*id);
                }
            }
            MirInstruction::Cast { value, .. } => {
                if let MirOperand::Local(id) = value {
                    vars.push(*id);
                }
            }
            MirInstruction::Drop { value, .. } => {
                if let MirOperand::Local(id) = value {
                    vars.push(*id);
                }
            }
            MirInstruction::Dup { src, .. } | MirInstruction::Reuse { src, .. } => {
                if let MirOperand::Local(id) = src {
                    vars.push(*id);
                }
            }
            MirInstruction::Alloc { .. } => {}
        }
        vars
    }
}

impl Default for ReachingDefinitions {
    fn default() -> Self {
        Self::new()
    }
}

impl DataflowAnalysis for ReachingDefinitions {
    type Domain = ReachingDefinitionsDomain;

    fn direction() -> AnalysisDirection {
        AnalysisDirection::Forward
    }

    fn entry_value() -> Self::Domain {
        ReachingDefinitionsDomain::top()
    }

    fn initial_value() -> Self::Domain {
        ReachingDefinitionsDomain::top()
    }

    fn transfer(&self, domain: &Self::Domain, instr: &MirInstruction) -> Self::Domain {
        let mut result = domain.clone();

        if let Some(def_var) = Self::get_defined_variable(instr) {
            result.definitions.retain(|d| d.variable != def_var);

            result.definitions.insert(Definition {
                variable: def_var,
                block_id: 0,
                instr_index: 0,
            });
        }

        result
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LivenessDomain {
    pub live: HashSet<MirLocalId>,
}

impl DataflowDomain for LivenessDomain {
    fn top() -> Self {
        Self {
            live: HashSet::new(),
        }
    }

    fn bottom() -> Self {
        Self {
            live: HashSet::new(),
        }
    }

    fn meet(&self, other: &Self) -> Self {
        Self {
            live: self.live.union(&other.live).copied().collect(),
        }
    }

    fn transfer(&self, _instr: &MirInstruction) -> Self {
        self.clone()
    }
}

#[derive(Debug, Clone)]
pub struct LivenessAnalysis {
    pub use_map: HashMap<MirBlockId, HashSet<MirLocalId>>,
    pub def_map: HashMap<MirBlockId, HashSet<MirLocalId>>,
}

impl LivenessAnalysis {
    pub fn new(cfg: &Cfg) -> Self {
        let mut use_map: HashMap<MirBlockId, HashSet<MirLocalId>> = HashMap::new();
        let mut def_map: HashMap<MirBlockId, HashSet<MirLocalId>> = HashMap::new();

        for (&block_id, block) in &cfg.blocks {
            let mut block_use = HashSet::new();
            let mut block_def = HashSet::new();

            for instr in &block.instructions {
                Self::analyze_instruction(instr, &mut block_use, &mut block_def);
            }

            if let MirTerminator::CondBranch { cond, .. } = &block.terminator {
                if let MirOperand::Local(id) = cond {
                    if !block_def.contains(id) {
                        block_use.insert(*id);
                    }
                }
            }

            if let MirTerminator::Return { value: Some(val) } = &block.terminator {
                if let MirOperand::Local(id) = val {
                    if !block_def.contains(id) {
                        block_use.insert(*id);
                    }
                }
            }

            use_map.insert(block_id, block_use);
            def_map.insert(block_id, block_def);
        }

        Self { use_map, def_map }
    }

    fn analyze_instruction(
        instr: &MirInstruction,
        block_use: &mut HashSet<MirLocalId>,
        block_def: &mut HashSet<MirLocalId>,
    ) {
        let used_vars = ReachingDefinitions::get_used_variables(instr);

        for var in used_vars {
            if !block_def.contains(&var) {
                block_use.insert(var);
            }
        }

        if let Some(def_var) = ReachingDefinitions::get_defined_variable(instr) {
            block_def.insert(def_var);
        }
    }

    pub fn analyze_function(
        &self,
        cfg: &Cfg,
    ) -> DataflowResult<DataflowAnalysisResult<LivenessDomain>> {
        let mut result = DataflowAnalysisResult {
            block_input: HashMap::new(),
            block_output: HashMap::new(),
        };

        let mut order = cfg.depth_first_order();
        order.reverse();

        for &block_id in &order {
            let output = {
                let succs = cfg.successors(block_id);
                if succs.is_empty() {
                    LivenessDomain::top()
                } else {
                    succs
                        .iter()
                        .filter_map(|s| result.block_input.get(s).cloned())
                        .fold(LivenessDomain::top(), |acc, d| acc.meet(&d))
                }
            };

            result.block_output.insert(block_id, output.clone());

            let block_use = self.use_map.get(&block_id).cloned().unwrap_or_default();
            let block_def = self.def_map.get(&block_id).cloned().unwrap_or_default();

            let mut live = output.live.clone();
            live = live.difference(&block_def).copied().collect();
            live = live.union(&block_use).copied().collect();

            result.block_input.insert(block_id, LivenessDomain { live });
        }

        Ok(result)
    }

    pub fn compute_live_ranges(
        &self,
        cfg: &Cfg,
    ) -> DataflowResult<HashMap<MirLocalId, Vec<(MirBlockId, usize)>>> {
        let result = self.analyze_function(cfg)?;

        let mut live_ranges: HashMap<MirLocalId, Vec<(MirBlockId, usize)>> = HashMap::new();

        for (&block_id, block) in &cfg.blocks {
            let live_in = result
                .block_input
                .get(&block_id)
                .cloned()
                .unwrap_or_default();
            let mut current_live = live_in.live.clone();

            for var in &current_live {
                live_ranges.entry(*var).or_default().push((block_id, 0));
            }

            for (idx, instr) in block.instructions.iter().enumerate() {
                if let Some(def_var) = ReachingDefinitions::get_defined_variable(instr) {
                    current_live.insert(def_var);
                    live_ranges
                        .entry(def_var)
                        .or_default()
                        .push((block_id, idx));
                }

                for var in ReachingDefinitions::get_used_variables(instr) {
                    current_live.insert(var);
                    live_ranges.entry(var).or_default().push((block_id, idx));
                }
            }
        }

        Ok(live_ranges)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AvailableExpressionsDomain {
    pub expressions: HashSet<Expression>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Expression {
    pub kind: ExpressionKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExpressionKind {
    BinaryOp {
        op: MirBinOp,
        left: MirOperand,
        right: MirOperand,
    },
    UnaryOp {
        op: MirUnOp,
        operand: MirOperand,
    },
    FieldAccess {
        object: MirOperand,
        field: String,
    },
}

impl DataflowDomain for AvailableExpressionsDomain {
    fn top() -> Self {
        Self {
            expressions: HashSet::new(),
        }
    }

    fn bottom() -> Self {
        Self {
            expressions: HashSet::new(),
        }
    }

    fn meet(&self, other: &Self) -> Self {
        Self {
            expressions: self
                .expressions
                .intersection(&other.expressions)
                .cloned()
                .collect(),
        }
    }

    fn transfer(&self, _instr: &MirInstruction) -> Self {
        self.clone()
    }
}

#[derive(Debug, Clone)]
pub struct AvailableExpressionsAnalysis;

impl AvailableExpressionsAnalysis {
    pub fn new() -> Self {
        Self
    }

    fn get_expression(instr: &MirInstruction) -> Option<Expression> {
        match instr {
            MirInstruction::BinaryOp {
                op, left, right, ..
            } => Some(Expression {
                kind: ExpressionKind::BinaryOp {
                    op: *op,
                    left: left.clone(),
                    right: right.clone(),
                },
            }),
            MirInstruction::UnaryOp { op, operand, .. } => Some(Expression {
                kind: ExpressionKind::UnaryOp {
                    op: *op,
                    operand: operand.clone(),
                },
            }),
            MirInstruction::FieldAccess { object, field, .. } => Some(Expression {
                kind: ExpressionKind::FieldAccess {
                    object: object.clone(),
                    field: field.clone(),
                },
            }),
            _ => None,
        }
    }

    fn kills_expression(instr: &MirInstruction, expr: &Expression) -> bool {
        let defined = ReachingDefinitions::get_defined_variable(instr);

        match &expr.kind {
            ExpressionKind::BinaryOp { left, right, .. } => defined.map_or(false, |d| {
                matches!(left, MirOperand::Local(id) if *id == d)
                    || matches!(right, MirOperand::Local(id) if *id == d)
            }),
            ExpressionKind::UnaryOp { operand, .. } => defined.map_or(
                false,
                |d| matches!(operand, MirOperand::Local(id) if *id == d),
            ),
            ExpressionKind::FieldAccess { object, .. } => defined.map_or(
                false,
                |d| matches!(object, MirOperand::Local(id) if *id == d),
            ),
        }
    }
}

impl Default for AvailableExpressionsAnalysis {
    fn default() -> Self {
        Self::new()
    }
}

impl DataflowAnalysis for AvailableExpressionsAnalysis {
    type Domain = AvailableExpressionsDomain;

    fn direction() -> AnalysisDirection {
        AnalysisDirection::Forward
    }

    fn entry_value() -> Self::Domain {
        AvailableExpressionsDomain::top()
    }

    fn initial_value() -> Self::Domain {
        AvailableExpressionsDomain {
            expressions: HashSet::new(),
        }
    }

    fn transfer(&self, domain: &Self::Domain, instr: &MirInstruction) -> Self::Domain {
        let mut result = domain.clone();

        for expr in domain.expressions.iter() {
            if Self::kills_expression(instr, expr) {
                result.expressions.remove(expr);
            }
        }

        if let Some(expr) = Self::get_expression(instr) {
            result.expressions.insert(expr);
        }

        result
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VeryBusyExpressionsDomain {
    pub expressions: HashSet<Expression>,
}

impl DataflowDomain for VeryBusyExpressionsDomain {
    fn top() -> Self {
        Self {
            expressions: HashSet::new(),
        }
    }

    fn bottom() -> Self {
        Self {
            expressions: HashSet::new(),
        }
    }

    fn meet(&self, other: &Self) -> Self {
        Self {
            expressions: self
                .expressions
                .intersection(&other.expressions)
                .cloned()
                .collect(),
        }
    }

    fn transfer(&self, _instr: &MirInstruction) -> Self {
        self.clone()
    }
}

#[derive(Debug, Clone)]
pub struct VeryBusyExpressionsAnalysis;

impl VeryBusyExpressionsAnalysis {
    pub fn new() -> Self {
        Self
    }
}

impl Default for VeryBusyExpressionsAnalysis {
    fn default() -> Self {
        Self::new()
    }
}

impl DataflowAnalysis for VeryBusyExpressionsAnalysis {
    type Domain = VeryBusyExpressionsDomain;

    fn direction() -> AnalysisDirection {
        AnalysisDirection::Backward
    }

    fn entry_value() -> Self::Domain {
        VeryBusyExpressionsDomain {
            expressions: HashSet::new(),
        }
    }

    fn initial_value() -> Self::Domain {
        VeryBusyExpressionsDomain {
            expressions: HashSet::new(),
        }
    }

    fn transfer(&self, domain: &Self::Domain, instr: &MirInstruction) -> Self::Domain {
        let mut result = domain.clone();

        for expr in domain.expressions.iter() {
            if AvailableExpressionsAnalysis::kills_expression(instr, expr) {
                result.expressions.remove(expr);
            }
        }

        if let Some(expr) = AvailableExpressionsAnalysis::get_expression(instr) {
            result.expressions.insert(expr);
        }

        result
    }
}

#[derive(Debug, Clone)]
pub struct DataflowSummary {
    pub block_count: usize,
    pub edge_count: usize,
    pub liveness: HashMap<MirBlockId, HashSet<MirLocalId>>,
    pub reaching_definitions: HashMap<MirBlockId, HashSet<Definition>>,
}

pub fn run_dataflow_analyses(cfg: &Cfg) -> DataflowResult<DataflowSummary> {
    let liveness = LivenessAnalysis::new(cfg);
    let liveness_result = liveness.analyze_function(cfg)?;

    let liveness_map: HashMap<MirBlockId, HashSet<MirLocalId>> = liveness_result
        .block_input
        .into_iter()
        .map(|(k, v)| (k, v.live))
        .collect();

    let reaching_defs = ReachingDefinitions::new();
    let rd_result = reaching_defs.analyze(cfg)?;

    let rd_map: HashMap<MirBlockId, HashSet<Definition>> = rd_result
        .block_output
        .into_iter()
        .map(|(k, v)| (k, v.definitions))
        .collect();

    Ok(DataflowSummary {
        block_count: cfg.block_count(),
        edge_count: cfg.edge_count(),
        liveness: liveness_map,
        reaching_definitions: rd_map,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cfg::CfgBuilder;

    #[test]
    fn liveness_domain_top_bottom() {
        let top = LivenessDomain::top();
        let bottom = LivenessDomain::bottom();

        assert!(top.live.is_empty());
        assert!(bottom.live.is_empty());
    }

    #[test]
    fn liveness_domain_meet() {
        let mut d1 = LivenessDomain::top();
        let mut d2 = LivenessDomain::top();

        d1.live.insert(1);
        d2.live.insert(2);

        let merged = d1.meet(&d2);

        assert!(merged.live.contains(&1));
        assert!(merged.live.contains(&2));
    }

    #[test]
    fn reaching_definitions_domain() {
        let mut d = ReachingDefinitionsDomain::top();
        d.definitions.insert(Definition {
            variable: 0,
            block_id: 0,
            instr_index: 0,
        });

        assert_eq!(d.definitions.len(), 1);
    }

    #[test]
    fn liveness_analysis_simple() {
        let mut builder = CfgBuilder::new();

        let local = builder.new_local(MirType::Int(32));

        builder.push_instruction(MirInstruction::Assign {
            dest: local,
            value: MirOperand::Constant(MirConstant::Int(42)),
        });

        let dest = builder.new_local(MirType::Int(32));
        builder.push_instruction(MirInstruction::BinaryOp {
            dest,
            op: MirBinOp::Add,
            left: MirOperand::Local(local),
            right: MirOperand::Constant(MirConstant::Int(1)),
        });

        let cfg = builder.build();

        let liveness = LivenessAnalysis::new(&cfg);
        let result = liveness
            .analyze_function(&cfg)
            .expect("analysis should succeed");

        assert!(!result.block_input.is_empty());
    }

    #[test]
    fn available_expressions_analysis() {
        let analysis = AvailableExpressionsAnalysis::new();

        let domain = AvailableExpressionsDomain {
            expressions: HashSet::new(),
        };

        let instr = MirInstruction::BinaryOp {
            dest: 0,
            op: MirBinOp::Add,
            left: MirOperand::Constant(MirConstant::Int(1)),
            right: MirOperand::Constant(MirConstant::Int(2)),
        };

        let new_domain = analysis.transfer(&domain, &instr);

        assert_eq!(new_domain.expressions.len(), 1);
    }

    #[test]
    fn dataflow_summary() {
        let builder = CfgBuilder::new();
        let cfg = builder.build();

        let summary = run_dataflow_analyses(&cfg).expect("analysis should succeed");

        assert_eq!(summary.block_count, 1);
        assert_eq!(summary.edge_count, 0);
    }

    #[test]
    fn expression_hash_and_eq() {
        let e1 = Expression {
            kind: ExpressionKind::BinaryOp {
                op: MirBinOp::Add,
                left: MirOperand::Constant(MirConstant::Int(1)),
                right: MirOperand::Constant(MirConstant::Int(2)),
            },
        };

        let e2 = Expression {
            kind: ExpressionKind::BinaryOp {
                op: MirBinOp::Add,
                left: MirOperand::Constant(MirConstant::Int(1)),
                right: MirOperand::Constant(MirConstant::Int(2)),
            },
        };

        assert_eq!(e1, e2);

        let mut set = HashSet::new();
        set.insert(e1.clone());
        set.insert(e2.clone());

        assert_eq!(set.len(), 1);
    }

    #[test]
    fn definition_struct() {
        let def = Definition {
            variable: 0,
            block_id: 1,
            instr_index: 2,
        };

        assert_eq!(def.variable, 0);
        assert_eq!(def.block_id, 1);
        assert_eq!(def.instr_index, 2);
    }

    #[test]
    fn liveness_use_def_analysis() {
        let mut builder = CfgBuilder::new();

        let x = builder.new_local(MirType::Int(32));
        let y = builder.new_local(MirType::Int(32));

        builder.push_instruction(MirInstruction::Assign {
            dest: x,
            value: MirOperand::Constant(MirConstant::Int(10)),
        });

        builder.push_instruction(MirInstruction::Assign {
            dest: y,
            value: MirOperand::Local(x),
        });

        let cfg = builder.build();

        let liveness = LivenessAnalysis::new(&cfg);

        let use_map = &liveness.use_map;
        let def_map = &liveness.def_map;

        assert!(!use_map.is_empty());
        assert!(!def_map.is_empty());
    }
}

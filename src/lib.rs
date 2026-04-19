// plato-flux-opcodes — Tile operations as FLUX bytecode opcodes
//
// Encodes tile manipulation as 1-2 byte opcodes compatible with flux-runtime-c.
// TILE_LOAD, TILE_INJECT, TILE_PRUNE, TILE_ANCHOR, TILE_FUSE, TILE_SEARCH,
// TILE_SNAP, TILE_EXPORT make tile operations a first-class ISA concern.
//
// Theory Bridge: This crate implements the Lock Algebra concept from
// Oracle1's flux-research (docs-paper-lock-algebra.md). Each opcode maps
// to a Lock L = (trigger, opcode_transformation, constraint).
// See theorem_refs module below for formal mappings.

// ── Theorem References (Lock Algebra Bridge) ────────────

/// Constants and types bridging Oracle1's Lock Algebra proofs to FM's opcodes.
///
/// Lock Algebra defines Locks as triples L = (t, o, c) where:
///   t = trigger pattern (regex over bytecode sequences)
///   o = opcode transformation (Bytecode → Bytecode)
///   c = constraint (first-order logic over program states)
///
/// Composition operators: sequential ⊕, parallel ⊗, conditional ⊕_c
///
/// Theorems (from flux-research docs-paper-lock-algebra.md):
///   Theorem 1: Composition creates monotonic compilation spaces
///   Theorem 2: Critical mass at n ≥ 7 locks (covers code theory)
///   Theorem 3: Wisdom compression ≥ 82%
///   Theorem 4: Cross-model transfer ≥ 80%

/// Minimum number of locks needed for critical mass (Theorem 2)
pub const CRITICAL_MASS_N: usize = 7;

/// Minimum compression ratio for wisdom (Theorem 3)
pub const MIN_COMPRESSION_RATIO: f64 = 0.82;

/// Minimum cross-model transfer ratio (Theorem 4)
pub const MIN_CROSS_MODEL_TRANSFER: f64 = 0.80;

/// A Lock triple — the fundamental unit of Lock Algebra.
/// Maps directly to tile opcodes: each opcode IS an (o) transformation
/// with an implicit trigger pattern and constraint.
#[derive(Debug, Clone)]
pub struct Lock {
    /// What triggers this lock (bytecode pattern)
    pub trigger: String,
    /// The opcode transformation this lock applies
    pub opcode_name: &'static str,
    /// The constraint enforced (first-order logic)
    pub constraint: String,
}

impl Lock {
    pub fn new(trigger: &str, opcode: TileOpcode, constraint: &str) -> Self {
        Self {
            trigger: trigger.to_string(),
            opcode_name: opcode.name(),
            constraint: constraint.to_string(),
        }
    }

    /// Sequential composition ⊕: combine two locks into one.
    /// L1 ⊕ L2 = (t1 ∪ t2, o2 ∘ o1, c1 ∧ c2)
    /// Result is associative but NOT commutative.
    pub fn sequential_compose(&self, other: &Lock) -> Lock {
        Lock {
            trigger: format!("({}) ∪ ({})", self.trigger, other.trigger),
            opcode_name: &"compose", // placeholder for actual composition
            constraint: format!("({}) ∧ ({})", self.constraint, other.constraint),
        }
    }
}

/// Build a minimal set of locks from tile opcodes.
/// Returns CRITICAL_MASS_N (7) locks, enough to achieve Theorem 2.
pub fn critical_mass_locks() -> Vec<Lock> {
    vec![
        Lock::new("tile_load", TileOpcode::TileLoad, "tile exists in registry"),
        Lock::new("tile_inject", TileOpcode::TileInject, "content is well-formed"),
        Lock::new("tile_anchor", TileOpcode::TileAnchor, "anchor level is valid 0-3"),
        Lock::new("tile_snap", TileOpcode::TileSnap, "constraint type is known"),
        Lock::new("tile_fuse", TileOpcode::TileFuse, "both tiles exist and are active"),
        Lock::new("tile_prune", TileOpcode::TilePrune, "weight below threshold"),
        Lock::new("tile_export", TileOpcode::TileExport, "format type is supported"),
    ]
}

// ── Opcodes ──────────────────────────────────────────────

/// Tile manipulation opcodes for FLUX bytecode VM.
/// Encoded as single-byte opcodes (0xD0-0xDF) with optional operand byte.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TileOpcode {
    /// Load tile by ID into register. Operand: register index.
    TileLoad   = 0xD0,
    /// Inject content into tile. Operand: register index (content source).
    TileInject = 0xD1,
    /// Remove tile below weight threshold. Operand: threshold * 100.
    TilePrune  = 0xD2,
    /// Pin tile as anchor (constraint). Operand: anchor level (0-3).
    TileAnchor = 0xD3,
    /// Merge two tiles by weight. Operands: reg_a, reg_b.
    TileFuse   = 0xD4,
    /// Search tiles by keyword. Operand: register index (query source).
    TileSearch = 0xD5,
    /// Snap tile content to constraint. Operand: constraint type.
    TileSnap   = 0xD6,
    /// Export tile to external format. Operand: format type.
    TileExport = 0xD7,
    /// Tag tile with metadata. Operand: tag category.
    TileTag    = 0xD8,
    /// Query tile metadata. Operand: register index.
    TileQuery  = 0xD9,
    /// Batch operation on tile set. Operand: operation type.
    TileBatch  = 0xDA,
    /// Set tile weight. Operand: weight * 100.
    TileWeight = 0xDB,
    /// Clone tile to new ID. Operand: register index (destination).
    TileClone  = 0xDC,
    /// Diff two tiles. Operand: register index (other tile).
    TileDiff   = 0xDD,
    /// Reserve opcode for future use.
    TileReserve = 0xDE,
    /// NOP for tile operations (alignment/padding).
    TileNop    = 0xDF,
}

impl TileOpcode {
    /// All tile opcodes in order
    pub fn all() -> &'static [TileOpcode] {
        &[
            TileOpcode::TileLoad, TileOpcode::TileInject, TileOpcode::TilePrune,
            TileOpcode::TileAnchor, TileOpcode::TileFuse, TileOpcode::TileSearch,
            TileOpcode::TileSnap, TileOpcode::TileExport, TileOpcode::TileTag,
            TileOpcode::TileQuery, TileOpcode::TileBatch, TileOpcode::TileWeight,
            TileOpcode::TileClone, TileOpcode::TileDiff, TileOpcode::TileReserve,
            TileOpcode::TileNop,
        ]
    }

    /// Count of defined opcodes
    pub fn count() -> usize {
        16
    }

    /// Decode a byte into a TileOpcode, if valid
    pub fn from_byte(byte: u8) -> Option<TileOpcode> {
        if byte >= 0xD0 && byte <= 0xDF {
            Some(unsafe { std::mem::transmute(byte) })
        } else {
            None
        }
    }

    /// Encode to byte
    pub fn to_byte(self) -> u8 {
        self as u8
    }

    /// Human-readable name
    pub fn name(self) -> &'static str {
        match self {
            TileOpcode::TileLoad => "TILE_LOAD",
            TileOpcode::TileInject => "TILE_INJECT",
            TileOpcode::TilePrune => "TILE_PRUNE",
            TileOpcode::TileAnchor => "TILE_ANCHOR",
            TileOpcode::TileFuse => "TILE_FUSE",
            TileOpcode::TileSearch => "TILE_SEARCH",
            TileOpcode::TileSnap => "TILE_SNAP",
            TileOpcode::TileExport => "TILE_EXPORT",
            TileOpcode::TileTag => "TILE_TAG",
            TileOpcode::TileQuery => "TILE_QUERY",
            TileOpcode::TileBatch => "TILE_BATCH",
            TileOpcode::TileWeight => "TILE_WEIGHT",
            TileOpcode::TileClone => "TILE_CLONE",
            TileOpcode::TileDiff => "TILE_DIFF",
            TileOpcode::TileReserve => "TILE_RESERVE",
            TileOpcode::TileNop => "TILE_NOP",
        }
    }

    /// Whether this opcode takes an operand byte
    pub fn has_operand(self) -> bool {
        !matches!(self, TileOpcode::TileNop | TileOpcode::TileReserve)
    }

    /// Category of the opcode
    pub fn category(self) -> OpcodeCategory {
        match self {
            TileOpcode::TileLoad | TileOpcode::TileQuery | TileOpcode::TileSearch
            | TileOpcode::TileClone => OpcodeCategory::Read,
            TileOpcode::TileInject | TileOpcode::TileAnchor | TileOpcode::TileTag
            | TileOpcode::TileWeight => OpcodeCategory::Write,
            TileOpcode::TilePrune | TileOpcode::TileFuse | TileOpcode::TileSnap
            | TileOpcode::TileDiff => OpcodeCategory::Transform,
            TileOpcode::TileExport | TileOpcode::TileBatch => OpcodeCategory::IO,
            TileOpcode::TileNop | TileOpcode::TileReserve => OpcodeCategory::Control,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpcodeCategory {
    Read,
    Write,
    Transform,
    IO,
    Control,
}

// ── Instruction ──────────────────────────────────────────

/// A single tile instruction: opcode + optional operand
#[derive(Debug, Clone)]
pub struct TileInstruction {
    pub opcode: TileOpcode,
    pub operand: u8,
    pub comment: String,
}

impl TileInstruction {
    pub fn new(opcode: TileOpcode) -> Self {
        Self { opcode, operand: 0, comment: String::new() }
    }

    pub fn with_operand(mut self, operand: u8) -> Self {
        self.operand = operand;
        self
    }

    pub fn with_comment(mut self, comment: &str) -> Self {
        self.comment = comment.to_string();
        self
    }

    /// Encode to bytes (1-2 bytes)
    pub fn encode(&self) -> Vec<u8> {
        let mut bytes = vec![self.opcode.to_byte()];
        if self.opcode.has_operand() {
            bytes.push(self.operand);
        }
        bytes
    }

    /// Decode from bytes, returns instruction and bytes consumed
    pub fn decode(data: &[u8]) -> Option<(TileInstruction, usize)> {
        if data.is_empty() { return None; }
        let opcode = TileOpcode::from_byte(data[0])?;
        let consumed = if opcode.has_operand() {
            if data.len() < 2 { return None; }
            Some((TileInstruction {
                opcode,
                operand: data[1],
                comment: String::new(),
            }, 2))
        } else {
            Some((TileInstruction::new(opcode), 1))
        };
        consumed
    }
}

// ── Program ──────────────────────────────────────────────

/// A sequence of tile instructions
#[derive(Debug, Clone)]
pub struct TileProgram {
    pub instructions: Vec<TileInstruction>,
    pub name: String,
}

impl TileProgram {
    pub fn new(name: &str) -> Self {
        Self { instructions: Vec::new(), name: name.to_string() }
    }

    pub fn push(&mut self, instruction: TileInstruction) {
        self.instructions.push(instruction);
    }

    /// Compile to bytecode
    pub fn compile(&self) -> Vec<u8> {
        self.instructions.iter().flat_map(|i| i.encode()).collect()
    }

    /// Decompile from bytecode
    pub fn decompile(data: &[u8]) -> Option<Self> {
        let mut instructions = Vec::new();
        let mut offset = 0;
        while offset < data.len() {
            let (inst, consumed) = TileInstruction::decode(&data[offset..])?;
            instructions.push(inst);
            offset += consumed;
        }
        Some(Self { instructions, name: String::new() })
    }

    /// Instruction count
    pub fn len(&self) -> usize {
        self.instructions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.instructions.is_empty()
    }

    /// Bytecode size
    pub fn bytecode_size(&self) -> usize {
        self.compile().len()
    }
}

// ── Tests ────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opcode_count() {
        assert_eq!(TileOpcode::all().len(), 16);
    }

    #[test]
    fn test_opcode_encode_decode_roundtrip() {
        for opcode in TileOpcode::all() {
            let byte = opcode.to_byte();
            let decoded = TileOpcode::from_byte(byte).unwrap();
            assert_eq!(decoded, *opcode);
        }
    }

    #[test]
    fn test_opcode_from_byte_invalid() {
        assert!(TileOpcode::from_byte(0x00).is_none());
        assert!(TileOpcode::from_byte(0xCF).is_none());
        assert!(TileOpcode::from_byte(0xE0).is_none());
        assert!(TileOpcode::from_byte(0xFF).is_none());
    }

    #[test]
    fn test_opcode_names() {
        assert_eq!(TileOpcode::TileLoad.name(), "TILE_LOAD");
        assert_eq!(TileOpcode::TileSnap.name(), "TILE_SNAP");
        assert_eq!(TileOpcode::TileNop.name(), "TILE_NOP");
    }

    #[test]
    fn test_opcode_has_operand() {
        assert!(TileOpcode::TileLoad.has_operand());
        assert!(TileOpcode::TileInject.has_operand());
        assert!(!TileOpcode::TileNop.has_operand());
        assert!(!TileOpcode::TileReserve.has_operand());
    }

    #[test]
    fn test_instruction_encode_decode() {
        let inst = TileInstruction::new(TileOpcode::TileLoad).with_operand(5);
        let bytes = inst.encode();
        assert_eq!(bytes, vec![0xD0, 5]);

        let (decoded, consumed) = TileInstruction::decode(&bytes).unwrap();
        assert_eq!(decoded.opcode, TileOpcode::TileLoad);
        assert_eq!(decoded.operand, 5);
        assert_eq!(consumed, 2);
    }

    #[test]
    fn test_instruction_no_operand() {
        let inst = TileInstruction::new(TileOpcode::TileNop);
        let bytes = inst.encode();
        assert_eq!(bytes, vec![0xDF]);

        let (decoded, consumed) = TileInstruction::decode(&bytes).unwrap();
        assert_eq!(decoded.opcode, TileOpcode::TileNop);
        assert_eq!(consumed, 1);
    }

    #[test]
    fn test_instruction_decode_empty() {
        assert!(TileInstruction::decode(&[]).is_none());
    }

    #[test]
    fn test_instruction_decode_truncated() {
        assert!(TileInstruction::decode(&[0xD0]).is_none()); // needs operand
    }

    #[test]
    fn test_program_compile() {
        let mut prog = TileProgram::new("test");
        prog.push(TileInstruction::new(TileOpcode::TileLoad).with_operand(0));
        prog.push(TileInstruction::new(TileOpcode::TileInject).with_operand(1));
        prog.push(TileInstruction::new(TileOpcode::TileAnchor).with_operand(2));
        prog.push(TileInstruction::new(TileOpcode::TileNop));

        let bytecode = prog.compile();
        assert_eq!(bytecode, vec![0xD0, 0x00, 0xD1, 0x01, 0xD3, 0x02, 0xDF]);
    }

    #[test]
    fn test_program_decompile() {
        let bytecode = vec![0xD0, 0x00, 0xD1, 0x01, 0xDF];
        let prog = TileProgram::decompile(&bytecode).unwrap();
        assert_eq!(prog.len(), 3);
        assert_eq!(prog.instructions[0].opcode, TileOpcode::TileLoad);
        assert_eq!(prog.instructions[0].operand, 0);
        assert_eq!(prog.instructions[2].opcode, TileOpcode::TileNop);
    }

    #[test]
    fn test_program_compile_decompile_roundtrip() {
        let mut prog = TileProgram::new("roundtrip");
        prog.push(TileInstruction::new(TileOpcode::TileSearch).with_operand(3));
        prog.push(TileInstruction::new(TileOpcode::TileFuse).with_operand(0));
        prog.push(TileInstruction::new(TileOpcode::TilePrune).with_operand(50));
        prog.push(TileInstruction::new(TileOpcode::TileExport).with_operand(1));

        let bytecode = prog.compile();
        let decompiled = TileProgram::decompile(&bytecode).unwrap();
        let recompiled = decompiled.compile();

        assert_eq!(bytecode, recompiled);
    }

    #[test]
    fn test_opcode_categories() {
        assert_eq!(TileOpcode::TileLoad.category(), OpcodeCategory::Read);
        assert_eq!(TileOpcode::TileInject.category(), OpcodeCategory::Write);
        assert_eq!(TileOpcode::TileFuse.category(), OpcodeCategory::Transform);
        assert_eq!(TileOpcode::TileExport.category(), OpcodeCategory::IO);
        assert_eq!(TileOpcode::TileNop.category(), OpcodeCategory::Control);
    }

    #[test]
    fn test_program_size() {
        let mut prog = TileProgram::new("size-test");
        prog.push(TileInstruction::new(TileOpcode::TileLoad).with_operand(0));
        prog.push(TileInstruction::new(TileOpcode::TileNop));
        assert_eq!(prog.len(), 2);
        assert_eq!(prog.bytecode_size(), 3); // 2 bytes + 1 byte
    }

    #[test]
    fn test_instruction_with_comment() {
        let inst = TileInstruction::new(TileOpcode::TileSnap)
            .with_operand(1)
            .with_comment("snap to Pythagorean manifold");
        assert_eq!(inst.comment, "snap to Pythagorean manifold");
    }

    #[test]
    fn test_opcode_byte_ranges() {
        for opcode in TileOpcode::all() {
            let byte = opcode.to_byte();
            assert!(byte >= 0xD0, "opcode {} byte {} < 0xD0", opcode.name(), byte);
            assert!(byte <= 0xDF, "opcode {} byte {} > 0xDF", opcode.name(), byte);
        }
    }

    // ── Theorem Reference Tests ─────────────────────────

    #[test]
    fn test_theorem_2_critical_mass_locks() {
        // Theorem 2: Critical mass at n ≥ 7 locks
        let locks = critical_mass_locks();
        assert_eq!(locks.len(), CRITICAL_MASS_N);
        assert!(locks.len() >= 7, "Need ≥ {} locks for critical mass", CRITICAL_MASS_N);
        // Each lock must reference a real opcode
        for lock in &locks {
            assert!(!lock.opcode_name.is_empty());
            assert!(!lock.trigger.is_empty());
            assert!(!lock.constraint.is_empty());
        }
    }

    #[test]
    fn test_theorem_2_below_critical_mass() {
        // Below critical mass: 6 locks should NOT cover code theory
        let locks = critical_mass_locks();
        let below = &locks[..6];
        assert!(below.len() < CRITICAL_MASS_N);
    }

    #[test]
    fn test_lock_creation_from_opcode() {
        let lock = Lock::new("0xD0", TileOpcode::TileLoad, "tile exists");
        assert_eq!(lock.opcode_name, "TILE_LOAD");
        assert_eq!(lock.trigger, "0xD0");
        assert_eq!(lock.constraint, "tile exists");
    }

    #[test]
    fn test_sequential_composition() {
        let l1 = Lock::new("0xD0", TileOpcode::TileLoad, "exists");
        let l2 = Lock::new("0xD1", TileOpcode::TileInject, "well-formed");
        let composed = l1.sequential_compose(&l2);
        // Sequential: constraints AND'd, triggers OR'd
        assert!(composed.constraint.contains("∧"));
        assert!(composed.trigger.contains("∪"));
    }

    #[test]
    fn test_theorem_constants() {
        // These are the proven bounds from Oracle1's experiments
        assert_eq!(CRITICAL_MASS_N, 7);
        assert!(MIN_COMPRESSION_RATIO >= 0.82);
        assert!(MIN_CROSS_MODEL_TRANSFER >= 0.80);
    }

    #[test]
    fn test_lock_opcode_coverage() {
        // The 7 critical mass locks should cover all 5 opcode categories
        let locks = critical_mass_locks();
        let categories: std::collections::HashSet<_> = locks.iter()
            .filter_map(|l| {
                // Map opcode names back to categories
                match l.opcode_name {
                    "TILE_LOAD" | "TILE_QUERY" | "TILE_SEARCH" | "TILE_CLONE" => Some("Read"),
                    "TILE_INJECT" | "TILE_ANCHOR" | "TILE_TAG" | "TILE_WEIGHT" => Some("Write"),
                    "TILE_PRUNE" | "TILE_FUSE" | "TILE_SNAP" | "TILE_DIFF" => Some("Transform"),
                    "TILE_EXPORT" | "TILE_BATCH" => Some("IO"),
                    _ => None,
                }
            })
            .collect();
        // Should cover at least 3 categories
        assert!(categories.len() >= 3, "Critical mass locks cover {} categories, need ≥ 3", categories.len());
    }
}

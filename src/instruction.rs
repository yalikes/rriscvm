use std::collections::BTreeMap;

const LOAD: u8 = 0b_00_000_11;
const LOAD_FP: u8 = 0b_00_001_11;
const CUSTOM_0: u8 = 0b_00_010_11;
const MISC_MEM: u8 = 0b_00_011_11;
const OP_IMM: u8 = 0b_00_100_11;
const AUIPC: u8 = 0b_00_101_11;
const OP_IMM_32: u8 = 0b_00_110_11;

type opcode = u8;
type funct3 = u8;
type funct7 = u8;
lazy_static! {
    pub static ref OP_IMM_Inst_type:BTreeMap<>={
        let mut m=BTreeMap::new();

        m
    };
}

pub enum InstructionTypes {
    R,
    I,
    S,
    B,
    U,
    J,
}

pub enum ItypeInstructionNames {
    ADDI,
    SLTI,
    SLTIU,
    ANDI,
    ORI,
    XORI,
}
pub struct ItypeInstruction {
    pub name: ItypeInstructionNames,
    pub opcode: u8,
    pub rd: u8,
    pub funct3: u8,
    pub rs1: u8,
    pub imm: u16,
}

impl ItypeInstruction {
    pub fn from_instruction(instruction: u32) {
        let opcode = instruction & 0b0111_1111; //
        match opcode as u8 {
            LOAD => { //how to refactor this?
            }
            LOAD_FP => {}
            CUSTOM_0 => {}
            MISC_MEM => {}
            OP_IMM => {}
            _ => {}
        }
    }
}

pub fn identify_instruction(instruction: u32) -> InstructionTypes {
    let opcode = instruction & 0b0111_1111;
    InstructionTypes::I
}

const LOAD: u8 = 0b_00_000_11;
const LOAD_FP: u8 = 0b_00_001_11;
const CUSTOM_0: u8 = 0b_00_010_11;
const MISC_MEM: u8 = 0b_00_011_11;
const OP_IMM: u8 = 0b_00_100_11;
const AUIPC: u8 = 0b_00_101_11;
const OP_IMM_32: u8 = 0b_00_110_11;

const ADDI_FUNCT3: u8 = 0b000;
const SLTI_FUNCT3: u8 = 0b010;
const SLTIU_FUNCT3: u8 = 0b011;
const XORI_FUNCT3: u8 = 0b100;
const ORI_FUNCT3: u8 = 0b110;
const ANDI_FUNCT3: u8 = 0b111;
const SLLI_FUNCT3: u8 = 0b001;
const SRLI_OR_SRAI_FUNCT3: u8 = 0b101;

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
    XORI,
    ORI,
    ANDI,
    SLLI,
    SRLI,
    SRAI,
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
    pub fn from_instruction(instruction: u32) -> ItypeInstruction {
        let opcode = (instruction & 0b0111_1111) as u8; //
        let rd = ((instruction >> 7) & 0b1_1111) as u8;
        let funct3 = ((instruction >> 12) & 0b11) as u8;
        let rs1 = ((instruction >> 15) & 0b1_1111) as u8;
        let imm = (instruction >> 20) as u16;
        let name: ItypeInstructionNames = match opcode as u8 {
            OP_IMM => match funct3 {
                ADDI_FUNCT3 => ItypeInstructionNames::ADDI,
                SLTI_FUNCT3 => ItypeInstructionNames::SLTI,
                SLTIU_FUNCT3 => ItypeInstructionNames::SLTIU,
                XORI_FUNCT3 => ItypeInstructionNames::XORI,
                ORI_FUNCT3 => ItypeInstructionNames::ORI,
                ANDI_FUNCT3 => ItypeInstructionNames::ANDI,
                SLLI_FUNCT3 => ItypeInstructionNames::SLLI,
                SRLI_OR_SRAI_FUNCT3 => {
                    if ((instruction >> 30) & 0b01) > 0 {
                        //check 30 bit
                        ItypeInstructionNames::SRLI
                    } else {
                        ItypeInstructionNames::SRAI
                    }
                }
                _ => {
                    panic!("not implement!")
                }
            },
            _ => {
                panic!("not implement!");
            }
        };
        ItypeInstruction {
            name,
            opcode,
            rd,
            funct3,
            rs1,
            imm,
        }
    }
}

pub fn identify_instruction(instruction: u32) -> InstructionTypes {
    let opcode = (instruction & 0b0111_1111) as u8;
    match opcode {
        OP_IMM => identify_inst_with_OP_IMM(instruction),
        _ => {
            panic!("not implement!")
        }
    }
}

fn identify_inst_with_OP_IMM(instruction: u32) -> InstructionTypes {
    InstructionTypes::I
}

#[test]
fn test_identify_instruction() {
    assert!(matches!(
        identify_instruction(0b1111_1111_1111_11111_000_11111_0010011),
        InstructionTypes::I
    ));
}

const LOAD: u8 = 0b00_000_11;
const LOAD_FP: u8 = 0b00_001_11;
const CUSTOM_0: u8 = 0b00_010_11;
const MISC_MEM: u8 = 0b00_011_11;
const OP_IMM: u8 = 0b00_100_11;
const AUIPC: u8 = 0b00_101_11;
const OP_IMM_32: u8 = 0b00_110_11;
const LUI: u8 = 0b01_101_11;
const OP: u8 = 0b01_100_11;
const JAL: u8 = 0b11_011_11;
const JALR: u8 = 0b11_001_11;
const BRANCH: u8 = 0b11_000_11;
const STORE: u8 = 0b01_000_11;

const ADDI_FUNCT3: u8 = 0b000;
const SLTI_FUNCT3: u8 = 0b010;
const SLTIU_FUNCT3: u8 = 0b011;
const XORI_FUNCT3: u8 = 0b100;
const ORI_FUNCT3: u8 = 0b110;
const ANDI_FUNCT3: u8 = 0b111;
const SLLI_FUNCT3: u8 = 0b001;
const SRLI_OR_SRAI_FUNCT3: u8 = 0b101;

const ADD_OR_SUB_FUNCT3: u8 = 0b000;
const SLL_FUNCT3: u8 = 0b001;
const SLT_FUNCT3: u8 = 0b010;
const SLTU_FUNCT3: u8 = 0b011;
const XOR_FUNCT3: u8 = 0b100;
const SRL_OR_SRA_FUNCT3: u8 = 0b101;
const OR_FUNCT3: u8 = 0b110;
const AND_FUNCT3: u8 = 0b111;

const BEQ_FUNCT3: u8 = 0b000;
const BNE_FUNCT3: u8 = 0b001;
const BLT_FUNCT3: u8 = 0b100;
const BGE_FUNCT3: u8 = 0b101;
const BLTU_FUNCT3: u8 = 0b110;
const BGEU_FUNCT3: u8 = 0b111;

const LB_FUNCT3: u8 = 0b000;
const LH_FUNCT3: u8 = 0b001;
const LW_FUNCT3: u8 = 0b010;
const LBU_FUNCT3: u8 = 0b100;
const LHU_FUNCT3: u8 = 0b101;

const SB: u8 = 0b000;
const SH: u8 = 0b001;
const SW: u8 = 0b010;

const ADD_FUNCT7: u8 = 0b000_0000;
const SUB_FUNCT7: u8 = 0b010_0000;
const SRL_FUNCT7: u8 = 0b000_0000;
const SRA_FUNCT7: u8 = 0b010_0000;

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
    JALR,
    LW,
    LH,
    LB,
    LBU,
    LHU,
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
        let opcode = (instruction & 0b0111_1111) as u8;
        let rd = ((instruction >> 7) & 0b1_1111) as u8;
        let funct3 = ((instruction >> 12) & 0b111) as u8;
        let rs1 = ((instruction >> 15) & 0b1_1111) as u8;
        let imm = (instruction >> 20) as u16;
        let name: ItypeInstructionNames = match opcode {
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
                        ItypeInstructionNames::SRAI
                    } else {
                        ItypeInstructionNames::SRLI
                    }
                }
                _ => {
                    panic!("not implement!")
                }
            },
            JALR => ItypeInstructionNames::JALR,
            LOAD => match funct3 {
                LB_FUNCT3 => ItypeInstructionNames::LB,
                LH_FUNCT3 => ItypeInstructionNames::LH,
                LW_FUNCT3 => ItypeInstructionNames::LW,
                LBU_FUNCT3 => ItypeInstructionNames::LBU,
                LHU_FUNCT3 => ItypeInstructionNames::LHU,
                _ => {
                    panic!("not implement!");
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

pub enum UtypeInstructionNames {
    LUI,
    AUIPC,
}

pub struct UtypeInstruction {
    pub name: UtypeInstructionNames,
    pub opcode: u8,
    pub rd: u8,
    pub imm: u32,
}

impl UtypeInstruction {
    pub fn from_instruction(instruction: u32) -> UtypeInstruction {
        let opcode = (instruction & 0b0111_1111) as u8;
        let rd = ((instruction >> 7) & 0b1_1111) as u8;
        let imm = (instruction >> 12) as u32;
        let name: UtypeInstructionNames = match opcode {
            LUI => UtypeInstructionNames::LUI,
            AUIPC => UtypeInstructionNames::AUIPC,
            _ => {
                panic!("not implement!");
            }
        };
        UtypeInstruction {
            name,
            opcode,
            rd,
            imm,
        }
    }
}

pub enum RtypeInstructionNames {
    ADD,
    SLT,
    SLTU,
    AND,
    OR,
    XOR,
    SLL,
    SRL,
    SUB,
    SRA,
}

pub struct RtypeInstruction {
    pub name: RtypeInstructionNames,
    pub opcode: u8,
    pub rs1: u8,
    pub rs2: u8,
    pub rd: u8,
    pub funct3: u8,
    pub funct7: u8,
}

impl RtypeInstruction {
    pub fn from_instruction(instruction: u32) -> RtypeInstruction {
        let opcode = (instruction & 0b0111_1111) as u8;
        let rd = ((instruction >> 7) & 0b1_1111) as u8;
        let funct3 = ((instruction >> 12) & 0b111) as u8;
        let rs1 = ((instruction >> 15) & 0b1_1111) as u8;
        let rs2 = ((instruction >> 20) & 0b1_1111) as u8;
        let funct7 = (instruction >> 25) as u8;
        let name: RtypeInstructionNames = match opcode {
            OP => match funct3 {
                ADD_OR_SUB_FUNCT3 => match funct7 {
                    ADD_FUNCT7 => RtypeInstructionNames::ADD,
                    SUB_FUNCT7 => RtypeInstructionNames::SUB,
                    _ => {
                        panic!("not implement");
                    }
                },
                SLL_FUNCT3 => RtypeInstructionNames::SLL,
                SLT_FUNCT3 => RtypeInstructionNames::SLT,
                SLTU_FUNCT3 => RtypeInstructionNames::SLTU,
                XOR_FUNCT3 => RtypeInstructionNames::XOR,
                SRL_OR_SRA_FUNCT3 => match funct7 {
                    SRL_FUNCT7 => RtypeInstructionNames::SRL,
                    SRA_FUNCT7 => RtypeInstructionNames::SRA,
                    _ => {
                        panic!("not implment");
                    }
                },
                OR_FUNCT3 => RtypeInstructionNames::OR,
                AND_FUNCT3 => RtypeInstructionNames::AND,
                _ => {
                    panic!("not implement!")
                }
            },
            _ => {
                panic!("not implement!")
            }
        };
        RtypeInstruction {
            name,
            opcode,
            rs1,
            rs2,
            rd,
            funct3,
            funct7,
        }
    }
}

pub enum JtypeInstructionNames {
    JAL,
}

pub struct JtypeInstruction {
    pub name: JtypeInstructionNames,
    pub opcode: u8,
    pub rd: u8,
    pub imm: u32,
}

impl JtypeInstruction {
    pub fn from_instruction(instruction: u32) -> JtypeInstruction {
        let opcode = (instruction & 0b0111_1111) as u8;
        let rd = ((instruction >> 7) & 0b1_1111) as u8;
        let imm = (instruction >> 12) as u32;
        let name = JtypeInstructionNames::JAL;
        JtypeInstruction {
            name,
            opcode,
            rd,
            imm,
        }
    }
}

pub enum BtypeInstructionNames {
    BEQ,
    BNE,
    BLT,
    BGE,
    BLTU,
    BGEU,
}

pub struct BtypeInstruction {
    pub name: BtypeInstructionNames,
    pub opcode: u8,
    pub rs1: u8,
    pub rs2: u8,
    pub funct3: u8,
    pub imm: u32,
}

impl BtypeInstruction {
    pub fn from_instruction(instruction: u32) -> BtypeInstruction {
        let opcode = (instruction & 0b0111_1111) as u8;
        let imm11 = ((instruction >> 7) & 0b1_1111) as u8;
        let funct3 = ((instruction >> 12) & 0b111) as u8;
        let rs1 = ((instruction >> 15) & 0b1_1111) as u8;
        let rs2 = ((instruction >> 20) & 0b1_1111) as u8;
        let imm5 = ((instruction >> 25) & 0b111_1111) as u8;
        let imm = imm11 as u32 | (imm5 as u32) << 5;
        let name = match funct3 {
            BEQ_FUNCT3 => BtypeInstructionNames::BEQ,
            BNE_FUNCT3 => BtypeInstructionNames::BNE,
            BLT_FUNCT3 => BtypeInstructionNames::BLT,
            BGE_FUNCT3 => BtypeInstructionNames::BGE,
            BLTU_FUNCT3 => BtypeInstructionNames::BLTU,
            BGEU_FUNCT3 => BtypeInstructionNames::BGEU,
            _ => {
                panic!("not implement");
            }
        };
        BtypeInstruction {
            name,
            opcode,
            rs1,
            rs2,
            funct3,
            imm,
        }
    }
}

pub enum StypeInstructionNames {
    SW,
    SH,
    SB,
}

pub struct StypeInstruction {
    pub name: StypeInstructionNames,
    pub opcode: u8,
    pub rs1: u8,
    pub rs2: u8,
    pub funct3: u8,
    pub imm: u16,
}

impl StypeInstruction {
    pub fn from_instruction(instruction: u32) -> StypeInstruction {
        let opcode = (instruction & 0b0111_1111) as u8;
        let imm0 = (instruction >> 7) & 0b1_1111;
        let funct3 = ((instruction >> 12) & 0b111) as u8;
        let rs1 = ((instruction >> 15) & 0b1_1111) as u8;
        let rs2 = ((instruction >> 20) & 0b1_1111) as u8;
        let imm5 = instruction >> 25;
        let imm = (imm5 << 5 | imm0) as u16;
        let name = match funct3 {
            SB => StypeInstructionNames::SB,
            SH => StypeInstructionNames::SH,
            SW => StypeInstructionNames::SW,
            _ => {
                panic!("not implement");
            }
        };
        StypeInstruction {
            name,
            opcode,
            rs1,
            rs2,
            funct3,
            imm
        }
    }
}

pub fn identify_instruction(instruction: u32) -> InstructionTypes {
    let opcode = (instruction & 0b0111_1111) as u8;
    match opcode {
        OP_IMM => identify_inst_with_op_imm(instruction),
        LUI => InstructionTypes::U,
        AUIPC => InstructionTypes::U,
        OP => InstructionTypes::R,
        JAL => InstructionTypes::J, 
        JALR => InstructionTypes::I,
        BRANCH => InstructionTypes::B,
        LOAD => InstructionTypes::I,
        STORE => InstructionTypes::S,
        _ => {
            panic!("not implement!")
        }
    }
}

fn identify_inst_with_op_imm(instruction: u32) -> InstructionTypes {
    InstructionTypes::I
}

#[test]
fn test_identify_instruction() {
    assert!(matches!(
        identify_instruction(0b1111_1111_1111_11111_000_11111_0010011),
        InstructionTypes::I
    ));
}

#[test]
fn test_itype_instruction() {
    let addi = ItypeInstruction::from_instruction(0b1111_1111_1111_11111_000_11111_0010011);
    assert!(matches!(addi.name, ItypeInstructionNames::ADDI));
    assert_eq!(addi.opcode, OP_IMM);
    assert_eq!(addi.funct3, ADDI_FUNCT3);
    assert_eq!(addi.rd, 0b1_1111);
    assert_eq!(addi.rs1, 0b1_1111);
    assert_eq!(addi.imm, 0b1111_1111_1111);
}

use crate::bit_utils::{b_sign_extend, i_sign_extend, j_sign_extend, u32_assemble, u_sign_extend};
use crate::instruction::{
    identify_instruction, BtypeInstruction, BtypeInstructionNames, InstructionTypes,
    ItypeInstruction, ItypeInstructionNames, JtypeInstruction, JtypeInstructionNames,
    RtypeInstruction, RtypeInstructionNames, UtypeInstruction, UtypeInstructionNames,
};
use crate::memory::Memory;

pub struct RiscvVirtualMachine {
    pub x0: u32,
    pub x1: u32,
    pub x2: u32,
    pub x3: u32,
    pub x4: u32,
    pub x5: u32,
    pub x6: u32,
    pub x7: u32,
    pub x8: u32,
    pub x9: u32,
    pub x10: u32,
    pub x11: u32,
    pub x12: u32,
    pub x13: u32,
    pub x14: u32,
    pub x15: u32,
    pub x16: u32,
    pub x17: u32,
    pub x18: u32,
    pub x19: u32,
    pub x20: u32,
    pub x21: u32,
    pub x22: u32,
    pub x23: u32,
    pub x24: u32,
    pub x25: u32,
    pub x26: u32,
    pub x27: u32,
    pub x28: u32,
    pub x29: u32,
    pub x30: u32,
    pub x31: u32,
    pub pc: u32,
    //pc indicate the address of current instruction.
    //for me, "current" means from fetch instruction(inclusive) to the before instrction finish.
    _ir: u32,
    pub memory: Memory,
}
impl RiscvVirtualMachine {
    pub fn new() -> RiscvVirtualMachine {
        RiscvVirtualMachine {
            x0: 0,
            x1: 0,
            x2: 0,
            x3: 0,
            x4: 0,
            x5: 0,
            x6: 0,
            x7: 0,
            x8: 0,
            x9: 0,
            x10: 0,
            x11: 0,
            x12: 0,
            x13: 0,
            x14: 0,
            x15: 0,
            x16: 0,
            x17: 0,
            x18: 0,
            x19: 0,
            x20: 0,
            x21: 0,
            x22: 0,
            x23: 0,
            x24: 0,
            x25: 0,
            x26: 0,
            x27: 0,
            x28: 0,
            x29: 0,
            x30: 0,
            x31: 0,
            pc: 0,
            _ir: 0,
            memory: Memory::new(),
        }
    }
    pub fn reset(&mut self) {
        self.memory = Memory::new();
        self.pc = 0;
        self.x0 = 0;
        self.x1 = 0;
        self.x2 = 0;
        self.x3 = 0;
        self.x4 = 0;
        self.x5 = 0;
        self.x6 = 0;
        self.x7 = 0;
        self.x8 = 0;
        self.x9 = 0;
        self.x10 = 0;
        self.x11 = 0;
        self.x12 = 0;
        self.x13 = 0;
        self.x14 = 0;
        self.x15 = 0;
        self.x16 = 0;
        self.x17 = 0;
        self.x18 = 0;
        self.x19 = 0;
        self.x20 = 0;
        self.x21 = 0;
        self.x22 = 0;
        self.x23 = 0;
        self.x24 = 0;
        self.x25 = 0;
        self.x26 = 0;
        self.x27 = 0;
        self.x28 = 0;
        self.x29 = 0;
        self.x30 = 0;
        self.x31 = 0;
        self._ir = 0;
    }
    pub fn fetch_instruction(&self) -> u32 {
        let x0 = self.memory.fetch(self.pc);
        let x1 = self.memory.fetch(self.pc + 1);
        let x2 = self.memory.fetch(self.pc + 2);
        let x3 = self.memory.fetch(self.pc + 3);
        u32_assemble(x0, x1, x2, x3)
    }
    pub fn exec(&mut self) {
        self._ir = self.fetch_instruction();
        match identify_instruction(self._ir) {
            InstructionTypes::R => {
                let inst = RtypeInstruction::from_instruction(self._ir);
                match inst.name {
                    RtypeInstructionNames::ADD => self.add(),
                    RtypeInstructionNames::SLT => self.slt(),
                    RtypeInstructionNames::SLTU => self.sltu(),
                    RtypeInstructionNames::AND => self.and(),
                    RtypeInstructionNames::OR => self.or(),
                    RtypeInstructionNames::XOR => self.xor(),
                    RtypeInstructionNames::SLL => self.sll(),
                    RtypeInstructionNames::SRL => self.srl(),
                    RtypeInstructionNames::SUB => self.sub(),
                    RtypeInstructionNames::SRA => self.sra(),
                }
            }
            InstructionTypes::I => {
                let inst = ItypeInstruction::from_instruction(self._ir);
                match inst.name {
                    ItypeInstructionNames::ADDI => self.addi(),
                    ItypeInstructionNames::SLTI => self.slti(),
                    ItypeInstructionNames::SLTIU => self.sltiu(),
                    ItypeInstructionNames::XORI => self.xori(),
                    ItypeInstructionNames::ORI => self.ori(),
                    ItypeInstructionNames::ANDI => self.andi(),
                    ItypeInstructionNames::SLLI => self.slli(),
                    ItypeInstructionNames::SRLI => self.srli(),
                    ItypeInstructionNames::SRAI => self.srai(),
                    ItypeInstructionNames::JALR => self.jalr(),
                }
            }

            InstructionTypes::S => {}

            InstructionTypes::B => {
                let inst = BtypeInstruction::from_instruction(self._ir);
                match inst.name {
                    BtypeInstructionNames::BEQ => self.beq(),
                    BtypeInstructionNames::BNE => self.bne(),
                    BtypeInstructionNames::BLT => self.blt(),
                    BtypeInstructionNames::BLTU => self.bltu(),
                    BtypeInstructionNames::BGE => self.bge(),
                    BtypeInstructionNames::BGEU => self.bgeu(),
                }
            }

            InstructionTypes::U => {
                let inst = UtypeInstruction::from_instruction(self._ir);
                match inst.name {
                    UtypeInstructionNames::LUI => self.lui(),
                    UtypeInstructionNames::AUIPC => self.auipc(),
                }
            }

            InstructionTypes::J => {
                let inst = JtypeInstruction::from_instruction(self._ir);
                match inst.name {
                    JtypeInstructionNames::JAL => self.jal(),
                }
            }
        }
        //change to next instruction here
    }

    pub fn addi(&mut self) {
        let inst = ItypeInstruction::from_instruction(self._ir);
        let imm = i_sign_extend(inst.imm) as i32;
        let rs = self.get_reg(inst.rs1) as i32;
        self.set_reg(inst.rd, (rs + imm) as u32);
        self.pc += 4;
    }

    pub fn slti(&mut self) {
        let inst = ItypeInstruction::from_instruction(self._ir);
        let imm = i_sign_extend(inst.imm);
        if (self.get_reg(inst.rs1) as i32) < imm as i32 {
            self.set_reg(inst.rd, 1);
        } else {
            self.set_reg(inst.rd, 0);
        }
        self.pc += 4;
    }

    pub fn sltiu(&mut self) {
        let inst = ItypeInstruction::from_instruction(self._ir);
        let imm = i_sign_extend(inst.imm);
        if self.get_reg(inst.rs1) < imm {
            self.set_reg(inst.rd, 1);
        } else {
            self.set_reg(inst.rd, 0);
        }
        self.pc += 4;
    }

    pub fn xori(&mut self) {
        let inst = ItypeInstruction::from_instruction(self._ir);
        let imm = i_sign_extend(inst.imm);
        let rs = self.get_reg(inst.rs1);
        self.set_reg(inst.rd, imm ^ rs);
        self.pc += 4;
    }

    pub fn ori(&mut self) {
        let inst = ItypeInstruction::from_instruction(self._ir);
        let imm = i_sign_extend(inst.imm);
        let rs = self.get_reg(inst.rs1);
        self.set_reg(inst.rd, imm | rs);
        self.pc += 4;
    }

    pub fn andi(&mut self) {
        let inst = ItypeInstruction::from_instruction(self._ir);
        let imm = i_sign_extend(inst.imm);
        let rs = self.get_reg(inst.rs1);
        self.set_reg(inst.rd, imm & rs);
        self.pc += 4;
    }

    pub fn slli(&mut self) {
        let inst = ItypeInstruction::from_instruction(self._ir);
        let shamt = inst.imm & 0b1_1111;
        let rs = self.get_reg(inst.rs1);
        self.set_reg(inst.rd, rs << shamt);
        self.pc += 4;
    }

    pub fn srli(&mut self) {
        let inst = ItypeInstruction::from_instruction(self._ir);
        let shamt = inst.imm & 0b1_1111;
        let rs: u32 = self.get_reg(inst.rs1);
        self.set_reg(inst.rd, rs >> shamt);
        self.pc += 4;
    }

    pub fn srai(&mut self) {
        let inst = ItypeInstruction::from_instruction(self._ir);
        let shamt = inst.imm & 0b1_1111;
        let rs: i32 = self.get_reg(inst.rs1) as i32;
        self.set_reg(inst.rd, (rs >> shamt) as u32);
        self.pc += 4;
    }

    pub fn lui(&mut self) {
        let inst = UtypeInstruction::from_instruction(self._ir);
        let imm = u_sign_extend(inst.imm);
        self.set_reg(inst.rd, imm);
        self.pc += 4;
    }

    pub fn auipc(&mut self) {
        let inst = UtypeInstruction::from_instruction(self._ir);
        let imm = u_sign_extend(inst.imm);
        self.set_reg(inst.rd, imm + self.pc);
        self.pc += 4;
    }

    pub fn add(&mut self) {
        let inst = RtypeInstruction::from_instruction(self._ir);
        let rs1 = self.get_reg(inst.rs1);
        let rs2 = self.get_reg(inst.rs2);
        self.set_reg(inst.rd, rs1 + rs2);
        self.pc += 4;
    }

    pub fn slt(&mut self) {
        let inst = RtypeInstruction::from_instruction(self._ir);
        let rs1 = self.get_reg(inst.rs1) as i32;
        let rs2 = self.get_reg(inst.rs2) as i32;
        if rs1 < rs2 {
            self.set_reg(inst.rd, 1);
        } else {
            self.set_reg(inst.rd, 0);
        }
        self.pc += 4;
    }

    pub fn sltu(&mut self) {
        let inst = RtypeInstruction::from_instruction(self._ir);
        let rs1 = self.get_reg(inst.rs1);
        let rs2 = self.get_reg(inst.rs2);
        if rs1 < rs2 {
            self.set_reg(inst.rd, 1);
        } else {
            self.set_reg(inst.rd, 0);
        }
        self.pc += 4;
    }

    pub fn and(&mut self) {
        let inst = RtypeInstruction::from_instruction(self._ir);
        let rs1 = self.get_reg(inst.rs1);
        let rs2 = self.get_reg(inst.rs2);
        self.set_reg(inst.rd, rs1 & rs2);
        self.pc += 4;
    }

    pub fn or(&mut self) {
        let inst = RtypeInstruction::from_instruction(self._ir);
        let rs1 = self.get_reg(inst.rs1);
        let rs2 = self.get_reg(inst.rs2);
        self.set_reg(inst.rd, rs1 | rs2);
        self.pc += 4;
    }

    pub fn xor(&mut self) {
        let inst = RtypeInstruction::from_instruction(self._ir);
        let rs1 = self.get_reg(inst.rs1);
        let rs2 = self.get_reg(inst.rs2);
        self.set_reg(inst.rd, rs1 ^ rs2);
        self.pc += 4;
    }

    pub fn sll(&mut self) {
        let inst = RtypeInstruction::from_instruction(self._ir);
        let rs1 = self.get_reg(inst.rs1);
        let rs2 = self.get_reg(inst.rs2) & 0b1_1111;
        self.set_reg(inst.rd, rs1 << rs2);
        self.pc += 4;
    }

    pub fn srl(&mut self) {
        let inst = RtypeInstruction::from_instruction(self._ir);
        let rs1 = self.get_reg(inst.rs1);
        let rs2 = self.get_reg(inst.rs2) & 0b1_1111;
        self.set_reg(inst.rd, rs1 >> rs2);
        self.pc += 4;
    }

    pub fn sub(&mut self) {
        let inst = RtypeInstruction::from_instruction(self._ir);
        let rs1 = self.get_reg(inst.rs1);
        let rs2 = self.get_reg(inst.rs2);
        self.set_reg(inst.rd, rs1 - rs2);
        self.pc += 4;
    }

    pub fn sra(&mut self) {
        let inst = RtypeInstruction::from_instruction(self._ir);
        let rs1 = self.get_reg(inst.rs1) as i32;
        let rs2 = (self.get_reg(inst.rs2) & 0b1_1111) as i32;
        self.set_reg(inst.rd, (rs1 >> rs2) as u32);
        self.pc += 4;
    }

    pub fn jal(&mut self) {
        let inst = JtypeInstruction::from_instruction(self._ir);
        let pc = self.pc;
        let imm = j_sign_extend(inst.imm) as i32;
        self.pc = ((pc as i32) + imm) as u32;
        self.set_reg(inst.rd, pc + 4);
    }

    pub fn jalr(&mut self) {
        let inst = ItypeInstruction::from_instruction(self._ir);
        let pc = self.pc;
        let imm = i_sign_extend(inst.imm) as i32;
        self.pc = ((self.get_reg(inst.rs1) as i32) + imm) as u32 & 0xff_ff_ff_fe;
        self.set_reg(inst.rd, pc + 4);
    }

    pub fn beq(&mut self) {
        let inst = BtypeInstruction::from_instruction(self._ir);
        let offset = b_sign_extend(inst.imm) as i32;
        let rs1 = self.get_reg(inst.rs1);
        let rs2 = self.get_reg(inst.rs2);
        if rs1 == rs2 {
            self.pc = ((self.pc as i32) + offset) as u32;
        } else {
            self.pc += 4;
        }
    }

    pub fn bne(&mut self) {
        let inst = BtypeInstruction::from_instruction(self._ir);
        let offset = b_sign_extend(inst.imm) as i32;
        let rs1 = self.get_reg(inst.rs1);
        let rs2 = self.get_reg(inst.rs2);
        if rs1 != rs2 {
            self.pc = ((self.pc as i32) + offset) as u32;
        } else {
            self.pc += 4;
        }
    }

    pub fn blt(&mut self) {
        let inst = BtypeInstruction::from_instruction(self._ir);
        let offset = b_sign_extend(inst.imm) as i32;
        let rs1 = self.get_reg(inst.rs1) as i32;
        let rs2 = self.get_reg(inst.rs2) as i32;
        if rs1 < rs2 {
            self.pc = ((self.pc as i32) + offset) as u32;
        } else {
            self.pc += 4;
        }
    }

    pub fn bltu(&mut self) {
        let inst = BtypeInstruction::from_instruction(self._ir);
        let offset = b_sign_extend(inst.imm) as i32;
        let rs1 = self.get_reg(inst.rs1);
        let rs2 = self.get_reg(inst.rs2);
        if rs1 < rs2 {
            self.pc = ((self.pc as i32) + offset) as u32;
        } else {
            self.pc += 4;
        }
    }

    pub fn bge(&mut self) {
        let inst = BtypeInstruction::from_instruction(self._ir);
        let offset = b_sign_extend(inst.imm) as i32;
        let rs1 = self.get_reg(inst.rs1) as i32;
        let rs2 = self.get_reg(inst.rs2) as i32;
        if rs1 >= rs2 {
            self.pc = ((self.pc as i32) + offset) as u32;
        } else {
            self.pc += 4;
        }
    }

    pub fn bgeu(&mut self) {
        let inst = BtypeInstruction::from_instruction(self._ir);
        let offset = b_sign_extend(inst.imm) as i32;
        let rs1 = self.get_reg(inst.rs1);
        let rs2 = self.get_reg(inst.rs2);
        if rs1 >= rs2 {
            self.pc = ((self.pc as i32) + offset) as u32;
        } else {
            self.pc += 4;
        }
    }

    pub fn get_reg(&self, reg_name: u8) -> u32 {
        match reg_name {
            0 => self.x0,
            1 => self.x1,
            2 => self.x2,
            3 => self.x3,
            4 => self.x4,
            5 => self.x5,
            6 => self.x6,
            7 => self.x7,
            8 => self.x8,
            9 => self.x9,
            10 => self.x10,
            11 => self.x11,
            12 => self.x12,
            13 => self.x13,
            14 => self.x14,
            15 => self.x15,
            16 => self.x16,
            17 => self.x17,
            18 => self.x18,
            19 => self.x19,
            20 => self.x20,
            21 => self.x21,
            22 => self.x22,
            23 => self.x23,
            24 => self.x24,
            25 => self.x25,
            26 => self.x26,
            27 => self.x27,
            28 => self.x28,
            29 => self.x29,
            30 => self.x30,
            31 => self.x31,
            _ => panic!("reg name incorrect"),
        }
    }
    pub fn set_reg(&mut self, reg_name: u8, value: u32) {
        match reg_name {
            0 => (),
            1 => self.x1 = value,
            2 => self.x2 = value,
            3 => self.x3 = value,
            4 => self.x4 = value,
            5 => self.x5 = value,
            6 => self.x6 = value,
            7 => self.x7 = value,
            8 => self.x8 = value,
            9 => self.x9 = value,
            10 => self.x10 = value,
            11 => self.x11 = value,
            12 => self.x12 = value,
            13 => self.x13 = value,
            14 => self.x14 = value,
            15 => self.x15 = value,
            16 => self.x16 = value,
            17 => self.x17 = value,
            18 => self.x18 = value,
            19 => self.x19 = value,
            20 => self.x20 = value,
            21 => self.x21 = value,
            22 => self.x22 = value,
            23 => self.x23 = value,
            24 => self.x24 = value,
            25 => self.x25 = value,
            26 => self.x26 = value,
            27 => self.x27 = value,
            28 => self.x28 = value,
            29 => self.x29 = value,
            30 => self.x30 = value,
            31 => self.x31 = value,
            _ => panic!("reg name incorrect"),
        }
    }
}

#[cfg(test)]
mod test_32i_isa {
    use super::RiscvVirtualMachine;

    #[test]
    fn test_addi() {
        let mut vm = RiscvVirtualMachine::new();
        vm.memory.write(3, 0b00000010u8);
        vm.memory.write(2, 0b10100001u8);
        vm.memory.write(1, 0b00000000u8);
        vm.memory.write(0, 0b10010011u8);

        vm.memory.write(7, 0b00000001u8);
        vm.memory.write(6, 0b01100000u8);
        vm.memory.write(5, 0b10000001u8);
        vm.memory.write(4, 0b00010011u8);

        vm.exec();
        assert_eq!(vm.x1, 42);
        vm.exec();
        assert_eq!(vm.x2, 64);
        assert_eq!(vm.pc, 8);
    }

    #[test]
    fn test_slti() {
        let mut vm = RiscvVirtualMachine::new();
        vm.memory.write(3, 0b00000000);
        vm.memory.write(2, 0b00110001);
        vm.memory.write(1, 0b00100000);
        vm.memory.write(0, 0b10010011);

        vm.memory.write(7, 0b00000000);
        vm.memory.write(6, 0b00110001);
        vm.memory.write(5, 0b00100000);
        vm.memory.write(4, 0b10010011);

        vm.memory.write(11, 0b11111111);
        vm.memory.write(10, 0b11110001);
        vm.memory.write(9, 0b00100000);
        vm.memory.write(8, 0b10010011);

        vm.exec();
        assert_eq!(vm.x1, 1);

        vm.set_reg(2, 4);
        vm.exec();
        assert_eq!(vm.x1, 0);

        vm.set_reg(2, 0);
        vm.exec();
        assert_eq!(vm.x1, 0);
    }

    #[test]
    fn test_sltiu() {
        let mut vm = RiscvVirtualMachine::new();
        vm.memory.write(3, 0b00000000u8);
        vm.memory.write(2, 0b00110001u8);
        vm.memory.write(1, 0b00110000u8);
        vm.memory.write(0, 0b10010011u8);

        vm.memory.write(7, 0b00000000u8);
        vm.memory.write(6, 0b00110001u8);
        vm.memory.write(5, 0b00110000u8);
        vm.memory.write(4, 0b10010011u8);

        vm.memory.write(11, 0b11111111u8);
        vm.memory.write(10, 0b11110001u8);
        vm.memory.write(9, 0b00110000u8);
        vm.memory.write(8, 0b10010011u8);

        vm.exec();
        assert_eq!(vm.x1, 1);

        vm.set_reg(2, 4);
        vm.exec();
        assert_eq!(vm.x1, 0);

        vm.exec();
        assert_eq!(vm.x1, 1);
    }

    #[test]
    fn test_andi() {
        /*
         * x2 = 6
         * imm = 12
         * 12 & 6 == 4
         */
        let mut vm = RiscvVirtualMachine::new();
        vm.memory.write(3, 0b00000000u8);
        vm.memory.write(2, 0b11000001u8);
        vm.memory.write(1, 0b01110000u8);
        vm.memory.write(0, 0b10010011u8);

        vm.set_reg(2, 6);
        vm.exec();
        assert_eq!(vm.get_reg(1), 4);
    }

    #[test]
    fn test_xori() {
        /*
         * x2 = 6
         * imm = 12
         * 12 ^ 6 == 10
         */
        let mut vm = RiscvVirtualMachine::new();
        vm.memory.write(3, 0b00000000u8);
        vm.memory.write(2, 0b11000001u8);
        vm.memory.write(1, 0b01000000u8);
        vm.memory.write(0, 0b10010011u8);

        vm.set_reg(2, 6);
        vm.exec();
        assert_eq!(vm.get_reg(1), 10);
    }

    #[test]
    fn test_ori() {
        /*
         * x2 = 6
         * imm = 12
         * 12 | 6 == 14
         */
        let mut vm = RiscvVirtualMachine::new();
        vm.memory.write(3, 0b00000000u8);
        vm.memory.write(2, 0b11000001u8);
        vm.memory.write(1, 0b01100000u8);
        vm.memory.write(0, 0b10010011u8);

        vm.set_reg(2, 6);
        vm.exec();
        assert_eq!(vm.get_reg(1), 14);
    }

    #[test]
    fn test_slli() {
        /*
         * x2 = 6
         * imm = 2
         * 6 << 2 == 24
         */
        let mut vm = RiscvVirtualMachine::new();
        vm.memory.write(3, 0b00000000u8);
        vm.memory.write(2, 0b00100001u8);
        vm.memory.write(1, 0b00010000u8);
        vm.memory.write(0, 0b10010011u8);

        vm.set_reg(2, 6);
        vm.exec();
        assert_eq!(vm.get_reg(1), 24);
    }

    #[test]
    fn test_srli() {
        /*
         * x2 = -10
         * imm = 1
         * -10  as u32 >> 1 == 2147483643
         */
        let mut vm = RiscvVirtualMachine::new();
        vm.memory.write(3, 0b00000000u8);
        vm.memory.write(2, 0b00010001u8);
        vm.memory.write(1, 0b01010000u8);
        vm.memory.write(0, 0b10010011u8);

        vm.set_reg(2, (-10 as i32) as u32);
        vm.exec();
        assert_eq!(vm.get_reg(1), 2147483643);
    }
    #[test]
    fn test_srai() {
        /*
         * x2 = -10
         * imm = 1
         * -10 >> 1 == -5
         */
        let mut vm = RiscvVirtualMachine::new();
        vm.memory.write(3, 0b01000000u8);
        vm.memory.write(2, 0b00010001u8);
        vm.memory.write(1, 0b01010000u8);
        vm.memory.write(0, 0b10010011u8);

        vm.set_reg(2, (-10 as i32) as u32);
        vm.exec();
        assert_eq!(vm.get_reg(1), (-5 as i32) as u32);
    }

    #[test]
    fn test_lui() {
        let mut vm = RiscvVirtualMachine::new();
        vm.memory.write(3, 0b00000000u8);
        vm.memory.write(2, 0b00000000u8);
        vm.memory.write(1, 0b11110000u8);
        vm.memory.write(0, 0b10110111u8);

        vm.exec();
        assert_eq!(vm.get_reg(1), 0b1111 << 12);
    }

    #[test]
    fn test_auipc() {
        let mut vm = RiscvVirtualMachine::new();
        vm.memory.write(3, 0b00000000u8);
        vm.memory.write(2, 0b00000000u8);
        vm.memory.write(1, 0b11110000u8);
        vm.memory.write(0, 0b10010111u8);

        vm.memory.write(7, 0b00000000u8);
        vm.memory.write(6, 0b00000000u8);
        vm.memory.write(5, 0b11110000u8);
        vm.memory.write(4, 0b10010111u8);

        vm.exec();
        assert_eq!(vm.get_reg(1), (0b1111 << 12) + 0);

        vm.exec();
        assert_eq!(vm.get_reg(1), (0b1111 << 12) + 4);
    }

    #[test]
    fn test_jal() {
        /*
         * pc = 0
         * imm = 8
         * 0 + 8 == 8
         */
        let mut vm = RiscvVirtualMachine::new();
        vm.memory.write(3, 0b00000000u8);
        vm.memory.write(2, 0b10000000u8);
        vm.memory.write(1, 0b00000000u8);
        vm.memory.write(0, 0b11101111u8);

        vm.memory.write(11, 0b11111111u8);
        vm.memory.write(10, 0b10011111u8);
        vm.memory.write(9, 0b11110000u8);
        vm.memory.write(8, 0b11101111u8);

        vm.exec();
        assert_eq!(vm.pc, 8);
        assert_eq!(vm.x1, 4);

        vm.exec();
        assert_eq!(vm.pc, 0);
        assert_eq!(vm.x1, 12);
    }

    #[test]
    fn test_jalr() {
        let mut vm = RiscvVirtualMachine::new();
        vm.memory.write(3, 0b00000000u8);
        vm.memory.write(2, 0b10010001u8);
        vm.memory.write(1, 0b00000000u8);
        vm.memory.write(0, 0b11100111u8);

        vm.memory.write(19, 0b11111111u8);
        vm.memory.write(18, 0b10000001u8);
        vm.memory.write(17, 0b00000000u8);
        vm.memory.write(16, 0b11100111u8);

        vm.set_reg(2, 8);
        vm.exec();
        assert_eq!(vm.pc, 16);
        assert_eq!(vm.x1, 4);

        vm.exec();
        assert_eq!(vm.pc, 0);
        assert_eq!(vm.x1, 20);
    }

    #[test]
    fn test_beq(){
        let mut vm = RiscvVirtualMachine::new();
        vm.memory.write(3, 0b00000000u8);
        vm.memory.write(2, 0b00110001u8);
        vm.memory.write(1, 0b00000100u8);
        vm.memory.write(0, 0b01100011u8);
        
        vm.set_reg(2, 12);
        vm.set_reg(3, 12);
        vm.exec();
        assert_eq!(vm.pc, 8);
    }

    
}

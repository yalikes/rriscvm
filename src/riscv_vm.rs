use crate::memory::Memory;
use crate::instruction::{InstructionTypes,ItypeInstructionNames ,identify_instruction};
use crate::bit_utils::u32_assemble;
pub struct RiscvVirtualMachine{
    pub x0:u32,
    pub x1:u32,
    pub x2:u32,
    pub x3:u32,
    pub x4:u32,
    pub x5:u32,
    pub x6:u32,
    pub x7:u32,
    pub x8:u32,
    pub x9:u32,
    pub x10:u32,
    pub x11:u32,
    pub x12:u32,
    pub x13:u32,
    pub x14:u32,
    pub x15:u32,
    pub x16:u32,
    pub x17:u32,
    pub x18:u32,
    pub x19:u32,
    pub x20:u32,
    pub x21:u32,
    pub x22:u32,
    pub x23:u32,
    pub x24:u32,
    pub x25:u32,
    pub x26:u32,
    pub x27:u32,
    pub x28:u32,
    pub x29:u32,
    pub x30:u32,
    pub x31:u32,
    pub pc:u32,
    //pc indicate the address of current instruction.
    //for me, "current" means from fetch instruction(inclusive) to the before instrction finish.
        _ir:u32,
    pub memory: Memory,
}
impl RiscvVirtualMachine{
    pub fn new()->RiscvVirtualMachine{
        RiscvVirtualMachine{
            x0:0,
            x1:0,
            x2:0,
            x3:0,
            x4:0,
            x5:0,
            x6:0,
            x7:0,
            x8:0,
            x9:0,
            x10:0,
            x11:0,
            x12:0,
            x13:0,
            x14:0,
            x15:0,
            x16:0,
            x17:0,
            x18:0,
            x19:0,
            x20:0,
            x21:0,
            x22:0,
            x23:0,
            x24:0,
            x25:0,
            x26:0,
            x27:0,
            x28:0,
            x29:0,
            x30:0,
            x31:0,
            pc:0,
            _ir:0,
            memory: Memory::new()
        }
    }
    pub fn reset(&mut self){
        self.memory=Memory::new();
        self.pc=0;
        self.x0=0;
        self.x1=0;
        self.x2=0;
        self.x3=0;
        self.x4=0;
        self.x5=0;
        self.x6=0;
        self.x7=0;
        self.x8=0;
        self.x9=0;
        self.x10=0;
        self.x11=0;
        self.x12=0;
        self.x13=0;
        self.x14=0;
        self.x15=0;
        self.x16=0;
        self.x17=0;
        self.x18=0;
        self.x19=0;
        self.x20=0;
        self.x21=0;
        self.x22=0;
        self.x23=0;
        self.x24=0;
        self.x25=0;
        self.x26=0;
        self.x27=0;
        self.x28=0;
        self.x29=0;
        self.x30=0;
        self.x31=0;
        self._ir=0;
    }
    pub fn fetch_instruction(&self) -> u32{
        let x0 = self.memory.fetch(self.pc);
        let x1 = self.memory.fetch(self.pc+1);
        let x2 = self.memory.fetch(self.pc+2);
        let x3 = self.memory.fetch(self.pc+3);
        u32_assemble(x0, x1, x2, x3)
    }
    pub fn exec(&mut self){
        self._ir=self.fetch_instruction();
        match identify_instruction(self._ir){
            InstructionTypes::R=>{

            }
            _=>{
            }
        }
        //change to next instruction here
    }
    pub fn ADDI(&mut self){

    }
    pub fn SLTI(&mut self){

    }
    pub fn SLTIU(&mut self){
        
    }
}

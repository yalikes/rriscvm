pub enum InstructionTypes{
    R,I,S,B,U,J
}

pub enum ItypeInstructionNames{
    ADDI,
    SLTI,SLTIU,
    ANDI,ORI,XORI
}
pub struct ItypeInstructions{
    instruction_name: ItypeInstructionNames,
}

pub fn identify_instruction(instruction:u32)->InstructionTypes{
   InstructionTypes::R
   //TODO here
}
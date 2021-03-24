mod riscv_vm;
mod memory;
mod instruction;
mod defines;

use riscv_vm::RiscvVirtualMachine;

fn main() {
    let mut vm = RiscvVirtualMachine::new();    
    initial(&mut vm);
}

fn initial(vm: &mut RiscvVirtualMachine){
    
}

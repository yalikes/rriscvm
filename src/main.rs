//#[macro_use]
//extern crate lazy_static;
//上面的那个库我已经忘了为什么要引入了

use rriscvm::riscv_vm;

use riscv_vm::RiscvVirtualMachine;

fn main() {
    let mut vm = RiscvVirtualMachine::new();    
    initial(&mut vm);
}

fn initial(vm: &mut RiscvVirtualMachine){
    
}

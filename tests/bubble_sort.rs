use rriscvm::riscv_vm;

use riscv_vm::RiscvVirtualMachine;

#[test]
fn test_bubble_sort() {
    let mut vm = RiscvVirtualMachine::new();
    vm.memory.write(3, 0b00001000u8); // load x2, x4+128
    vm.memory.write(2, 0b00000010u8);
    vm.memory.write(1, 0b00000001u8);
    vm.memory.write(0, 0b00000011u8);

    vm.memory.write(7, 0b00001000u8); // load x3, x4+129
    vm.memory.write(6, 0b00010010u8);
    vm.memory.write(5, 0b00000001u8);
    vm.memory.write(4, 0b10000011u8);

    vm.memory.write(11, 0b00000000u8); //blt x3,x2,8
    vm.memory.write(10, 0b00100001u8);
    vm.memory.write(9, 0b11000100u8);
    vm.memory.write(8, 0b01100011u8);

    vm.memory.write(15, 0b00000010u8); // move(addi) x6, 42
    vm.memory.write(14, 0b10100000u8);
    vm.memory.write(13, 0b00000011u8);
    vm.memory.write(12, 0b00010011u8);

    vm.memory.write(19, 0b00000000u8); // move x5, x3
    vm.memory.write(18, 0b00000001u8);
    vm.memory.write(17, 0b10000010u8);
    vm.memory.write(16, 0b10010011u8);

    vm.memory.write(23, 0b00000000u8); // move x3, x2
    vm.memory.write(22, 0b00000001u8);
    vm.memory.write(21, 0b00000001u8);
    vm.memory.write(20, 0b10010011u8);

    vm.memory.write(27, 0b00000000u8); // move x2, x5
    vm.memory.write(26, 0b00000010u8);
    vm.memory.write(25, 0b10000001u8);
    vm.memory.write(24, 0b00010011u8);

    let v = vec![42, 26, 90, 12, 0, 1, 99, 33];
    for i in 0..8 {
        vm.memory.write(128 + i, v[i as usize]);
    }
    vm.exec();
    vm.exec();
    vm.exec();

    vm.exec();
    vm.exec();
    vm.exec();

    assert_eq!(vm.get_reg(2), 26);
    assert_eq!(vm.get_reg(3), 42);
    assert_eq!(vm.get_reg(5), 26);
}

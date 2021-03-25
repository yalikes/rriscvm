pub fn u32_assemble(x0:u8,x1:u8,x2:u8,x3:u8) -> u32{
    // x0: LSB, x3:MSB
    ((x3 as u32)<<24) |
    ((x2 as u32)<<16) |
    ((x1 as u32)<<8)  |
    (x0 as u32)
}

#[test]
fn test_u32_assemble() {
    assert_eq!(u32_assemble(1, 2, 3, 4),0x_04_03_02_01);
}
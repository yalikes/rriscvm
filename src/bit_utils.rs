pub fn u32_assemble(x0: u8, x1: u8, x2: u8, x3: u8) -> u32 {
    // x0: LSB, x3:MSB
    ((x3 as u32) << 24) |
    ((x2 as u32) << 16) |
    ((x1 as u32) << 8)  |
    (x0 as u32)
}
pub fn sign_extend12(imm: u16) -> i32 {
    ((((imm & 0x0fff) >> 11) as u32) * 0xffff_f800u32 | ((imm & 0x07ff) as u32)) as i32
}

#[test]
fn test_u32_assemble() {
    assert_eq!(u32_assemble(1, 2, 3, 4), 0x_04_03_02_01);
}

#[test]
fn test_sign_extend() {
    assert_eq!(sign_extend12(0x0fff), -1i32);
    assert_eq!(sign_extend12(0x0800), -2048i32);
    assert_eq!(sign_extend12(0x07ff), 2047i32);
}

pub fn u32_assemble(x0: u8, x1: u8, x2: u8, x3: u8) -> u32 {
    // x0: LSB, x3:MSB
    ((x3 as u32) << 24) |
    ((x2 as u32) << 16) |
    ((x1 as u32) << 8)  |
    (x0 as u32)
}
pub fn i_sign_extend(imm: u16) -> u32 {
    (((imm & 0x0fff) >> 11) as u32) * 0xffff_f800u32 | ((imm & 0x07ff) as u32)
}

pub fn u_sign_extend(imm: u32) -> u32 {
    imm << 12
}

pub fn j_sign_extend(imm: u32) -> u32 {
    let imm12 = imm & 0b1111_1111;
    let imm11 = (imm >> 8) &0b1;
    let imm1 = (imm >> 9) &0b11_1111_1111;
    let imm20 = (imm >> 19) &0b1;
    imm20 * 0xfff0_0000 |  imm12 << 12 | imm11 << 11 |
    imm1 << 1
}

pub fn b_sign_extend(imm: u32) -> u32{
    let imm11 = imm & 0b1;
    let imm1 = (imm >> 1) & 0b1111;
    let imm5 = (imm >> 5) & 0b11_1111;
    let imm12 = (imm >> 11) & 0b1;
    imm12 * 0xff_ff_f0_00 | imm11 << 11 | imm5 << 5 | imm1 << 1
}

#[test]
fn test_u32_assemble() {
    assert_eq!(u32_assemble(1, 2, 3, 4), 0x_04_03_02_01);
}

#[test]
fn test_sign_extend() {
    assert_eq!(i_sign_extend(0x0fff), (-1i32) as u32);
    assert_eq!(i_sign_extend(0x0800), (-2048i32) as u32);
    assert_eq!(i_sign_extend(0x07ff), (2047i32) as u32);
}

#[test]
fn test_j_sign_extend() {
    assert_eq!(j_sign_extend(0b1000_1111_0000_1111_0000), 0b1111_1111_1111_1111_0000_0000_1111_0000);
}

#[test]
fn test_u_sign_extend() {
    assert_eq!(b_sign_extend(0b1_010101_1111_1),
    0b1111_1111_1111_1111_1111_1_010101_1111_0
);
}

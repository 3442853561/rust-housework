trait ToU8<T> {
    fn to_u8(self) -> T;
}

impl ToU8<(u8, u8)> for u16 {
    fn to_u8(self) -> (u8, u8) {
        ((self >> 8) as u8,
         self as u8)
    }
}

impl ToU8<(u8, u8, u8, u8)> for u32 {
    fn to_u8(self) -> (u8, u8, u8, u8) {
        ((self >> 24) as u8,
         (self >> 16) as u8,
         (self >> 8) as u8,
         self as u8)
    }
}

impl ToU8<(u8, u8, u8, u8, u8, u8, u8, u8)> for u64 {
    fn to_u8(self) -> (u8, u8, u8, u8, u8, u8, u8, u8) {
        ((self >> 56) as u8,
         (self >> 48) as u8,
         (self >> 40) as u8,
         (self >> 32) as u8,
         (self >> 24) as u8,
         (self >> 16) as u8,
         (self >> 8) as u8,
         self as u8)
    }
}


fn main() {
    let foo: u64 = 256 * 256 * 256;
    println!("{:?}", foo.to_u8());
}

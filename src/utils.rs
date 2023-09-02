pub mod bits {
    #[inline]
    pub fn get_bit(input: u8, offset: u8) -> bool {
        (input >> offset) & 1 != 0
    }
}

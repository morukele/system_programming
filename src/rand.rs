fn mock_rand(n: u8) -> f32 {
    let base: u32 = 0b0011_1111_0000_0000_0000_0000_0000_0000;
    let large_n = (n as u32) << 15;
    let f32_bits = base | large_n;
    let m = f32::from_bits(f32_bits);
    2.0 * (m - 0.5)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_an_f32() {
        assert_eq!(mock_rand(0xff), 0.99609375);
        assert_eq!(mock_rand(0x7f), 0.49609375);
        assert_eq!(mock_rand(0x00), 0.0);
    }
}

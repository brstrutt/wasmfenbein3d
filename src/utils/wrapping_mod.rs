pub fn wrapping_mod(val: isize, modulus: isize) -> usize {
    let mut val = val % modulus;
    if val < 0 {
        val = modulus + val;
    }
    val as usize
}

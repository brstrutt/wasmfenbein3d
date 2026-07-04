pub fn wrapping_mod(val: f64, modulus: f64) -> f64 {
    let mut val = val % modulus;
    if val < 0.0 {
        val = modulus + val;
    }
    val
}

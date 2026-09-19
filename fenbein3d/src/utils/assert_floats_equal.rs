#[cfg(test)]
pub mod tests {
    pub fn assert_floats_equal(left: f64, right: f64) {
        let big = left.max(right);
        let small = left.min(right);

        assert!(
            big - small < 0.000_000_000_1,
            "{} does not equal {}",
            left,
            right
        );
    }
}

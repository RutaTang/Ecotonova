use num_traits::Float;

pub fn float_mod<T: Float>(a: T, b: T) -> T {
    a - b * (a / b).floor()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_float_mod() {
        assert_eq!(float_mod(5.0, 3.0), 2.0);
        assert_eq!(float_mod(-5.0, 3.0), 1.0);
        assert_eq!(float_mod(5.0, -3.0), -1.0);
        assert_eq!(float_mod(-5.0, -3.0), -2.0);
    }
}
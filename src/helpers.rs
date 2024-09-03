pub fn convert_float_to_string(val: f32, precision: usize) -> String {
    format!("{:.prec$}", val, prec = precision)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_float_to_string() {
        assert_eq!(convert_float_to_string(3.14159, 2), "3.14");
        assert_eq!(convert_float_to_string(3.14159, 4), "3.1416");
        assert_eq!(convert_float_to_string(3.0, 2), "3.00");
        assert_eq!(convert_float_to_string(0.123456, 3), "0.123");
        assert_eq!(convert_float_to_string(1.0, 0), "1");
    }

    #[test]
    fn test_convert_float_to_string_zero() {
        assert_eq!(convert_float_to_string(0.0, 2), "0.00");
    }

    #[test]
    fn test_convert_float_to_string_negative() {
        assert_eq!(convert_float_to_string(-3.14159, 2), "-3.14");
    }
}
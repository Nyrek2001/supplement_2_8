/// Adds two 64-bit floating point numbers.
#[no_mangle]
pub extern "C" fn add(a: f64, b: f64) -> f64 {
    a + b
}

/// Subtracts the second 64-bit floating point number from the first.
#[no_mangle]
pub extern "C" fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

/// Multiplies two 64-bit floating point numbers.
#[no_mangle]
pub extern "C" fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

/// Divides the first 64-bit floating point number by the second.
/// Returns NaN if dividing by zero.
#[no_mangle]
pub extern "C" fn divide(a: f64, b: f64) -> f64 {
    a / b
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1.0, 2.0), 3.0);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(5.0, 3.0), 2.0);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(2.0, 4.0), 8.0);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(10.0, 2.0), 5.0);
    }
}


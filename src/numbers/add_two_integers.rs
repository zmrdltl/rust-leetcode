/// Adds two integers with overflow checking
///
/// # Arguments
///
/// * `num1` - First integer
/// * `num2` - Second integer
///
/// # Returns
///
/// * `Result<i32, &'static str>` - The sum of num1 and num2, or an error if overflow occurs
///
/// # Errors
///
/// Returns an error if the addition would cause an integer overflow.
///
/// # Examples
///
/// ```
/// use rust_leetcode::numbers::sum;
///
/// assert_eq!(sum(2, 3), Ok(5));
/// ```
pub fn sum(num1: i32, num2: i32) -> Result<i32, &'static str> {
    num1.checked_add(num2).ok_or("Integer overflow occurred")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        assert_eq!(sum(1, 2), Ok(3));
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(sum(i32::MIN, 0), Ok(i32::MIN));
        assert_eq!(sum(i32::MAX, 0), Ok(i32::MAX));

        assert!(sum(i32::MAX, 1).is_err());
        assert!(sum(i32::MIN, -1).is_err());
    }
}

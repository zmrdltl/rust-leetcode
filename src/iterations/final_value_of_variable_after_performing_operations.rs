/// Final value of variable after performing operations
///
/// # Arguments
///
/// * `operations` - Operations to perform
///
/// # Returns
///
/// * `Result<i32, &'static str>` - The sum of num1 and num2, or an error if overflow occurs
///
/// # Errors
///
/// Returns an error if
/// - Length of operations is not between 1 and 100
/// - Operations contains invalid operations
///
/// # Examples
///
/// ```
/// use rust_leetcode::iterations::final_value_after_operations;
///
/// assert_eq!(final_value_after_operations(&["X++", "++X", "--X", "X--"]), Ok(0));
/// ```
pub fn final_value_after_operations(operations: &[&str]) -> Result<i32, &'static str> {
    if operations.is_empty() || operations.len() > 100 {
        return Err("Length of operations is not between 1 and 100");
    }

    operations
        .iter()
        .try_fold(0, |acc, &operation| match operation {
            "X++" | "++X" => Ok(acc + 1),
            "X--" | "--X" => Ok(acc - 1),
            _ => Err("Invalid operation"),
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        assert_eq!(
            final_value_after_operations(&["X++", "++X", "--X", "X--"]),
            Ok(0)
        );
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(
            final_value_after_operations(&[]),
            Err("Length of operations is not between 1 and 100")
        );
        assert_eq!(
            final_value_after_operations(&["X+++"]),
            Err("Invalid operation")
        );
    }
}

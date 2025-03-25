/// Shuffle String
///
/// # Arguments
///
/// * `s` - The string to shuffle
/// * `indices` - The indices to shuffle the string
///
/// # Returns
///
/// * `Result<String, &'static str>` - The shuffled string, or an error if the length of `s` is not equal to the length of `indices`
///
/// # Errors
///
/// Returns an error if
/// - Length of `s` is not equal to the length of `indices`
/// - Length of `indices` is not between 1 and 100
/// - Elements of `indices` is not between 0 and `s.len() - 1`
///
/// # Panics
///
/// This function should not panic as all error conditions are handled with Results.
///
/// # Examples
///
/// ```
/// use rust_leetcode::encoding_decoding::restore_string;
///
/// assert_eq!(restore_string("codeleet", &[4,5,6,7,0,2,1,3]), Ok("leetcode".to_string()));
/// ```
pub fn restore_string(s: &str, indices: &[usize]) -> Result<String, &'static str> {
    if s.len() != indices.len() {
        return Err("Length of s must be equal to the length of indices");
    }
    if indices.is_empty() || indices.len() > 100 {
        return Err("Length of indices must be between 1 and 100");
    }
    if indices.iter().any(|&element| element >= s.len()) {
        return Err("Elements of indices must be between 0 and s.len() - 1");
    }

    let mut pairs: Vec<_> = s.chars().zip(indices.iter()).collect();
    pairs.sort_unstable_by_key(|&(_, index)| index);

    Ok(pairs.into_iter().map(|(c, _)| c).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        assert_eq!(
            restore_string("codeleet", &[4, 5, 6, 7, 0, 2, 1, 3]),
            Ok("leetcode".to_string())
        );
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(
            restore_string("a", &[1, 2]),
            Err("Length of s must be equal to the length of indices")
        );
        assert_eq!(
            restore_string("", &[]),
            Err("Length of indices must be between 1 and 100")
        );
        assert_eq!(
            restore_string("ab", &[0, 2]),
            Err("Elements of indices must be between 0 and s.len() - 1")
        );
    }
}

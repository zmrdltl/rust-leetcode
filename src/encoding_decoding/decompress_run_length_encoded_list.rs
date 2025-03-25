use std::iter::repeat;

/// Decompress Run-Length Encoded List
///
/// # Arguments
///
/// * `nums` - The run-length encoded list
///
/// # Returns
///
/// * `Result<Vec<i32>, &'static str>` - The decompressed list, or an error if overflow occurs
///
/// # Errors
///
/// Returns an error if
/// - Length of `nums` is not between 2 and 100
/// - Length of `nums` is odd
/// - Elements of `nums` is not between 1 and 100 for all `i % 2 == 0`
///
/// # Examples
///
/// ```
/// use rust_leetcode::encoding_decoding::decompress_rl_elist;
///
/// assert_eq!(decompress_rl_elist(&[1, 2, 3, 4]), Ok(vec![2, 4, 4, 4]));
/// ```
pub fn decompress_rl_elist(nums: &[usize]) -> Result<Vec<usize>, &'static str> {
    if nums.len() < 2 || nums.len() > 100 {
        return Err("Length of nums must be between 2 and 100");
    }
    if nums.len() % 2 != 0 {
        return Err("Length of nums must be even");
    }
    if nums.iter().any(|&x| !(1..=100).contains(&x)) {
        return Err("Elements of nums must be between 1 and 100");
    }

    Ok(nums
        .chunks(2)
        .flat_map(|chunk| repeat(chunk[1]).take(chunk[0]))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        assert_eq!(decompress_rl_elist(&[1, 2, 3, 4]), Ok(vec![2, 4, 4, 4]));
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(
            decompress_rl_elist(&[]),
            Err("Length of nums must be between 2 and 100")
        );
        assert_eq!(
            decompress_rl_elist(&[1, 2, 3]),
            Err("Length of nums must be even")
        );
        assert_eq!(
            decompress_rl_elist(&[0, 1]),
            Err("Elements of nums must be between 1 and 100")
        );
    }
}

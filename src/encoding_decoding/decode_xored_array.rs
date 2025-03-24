use std::iter::once;

/// Decode `XORed` Array
///
/// # Arguments
///
/// * `encoded` - The encoded array
/// * `first` - The first integer
///
/// # Returns
///
/// * `Result<Vec<i32>, &'static str>` - The decoded array, or an error if overflow occurs
///
/// # Errors
///
/// Returns an error if
/// - `encoded` is empty
/// - Elements of `encoded` are not in the range of 0 to 10^5
/// - `first` is not in the range of 0 to 10^5
///
/// # Examples
///
/// ```
/// use rust_leetcode::encoding_decoding::decode;
///
/// assert_eq!(decode(&[1, 2, 3], 1), Ok(vec![1, 0, 2, 1]));
/// ```
pub fn decode(encoded: &[i32], first: i32) -> Result<Vec<i32>, &'static str> {
    if encoded.is_empty() || encoded.len() > 100_000 {
        return Err("Encoded length must be between 1 and 100,000");
    }
    if !(0..=100_000).contains(&first) {
        return Err("First must be between 0 and 100,000");
    }
    if encoded.iter().any(|&x| !(0..=100_000).contains(&x)) {
        return Err("Encoded elements must be between 0 and 100,000");
    }

    Ok(once(first)
        .chain(encoded.iter().scan(first, |acc, &x| {
            *acc ^= x;
            Some(*acc)
        }))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        assert_eq!(decode(&[1, 2, 3], 1), Ok(vec![1, 0, 2, 1]));
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(
            decode(&[], 0),
            Err("Encoded length must be between 1 and 100,000")
        );
        assert_eq!(
            decode(&[1, 2, 3], 100_001),
            Err("First must be between 0 and 100,000")
        );
        assert_eq!(
            decode(&[100_001], 10),
            Err("Encoded elements must be between 0 and 100,000")
        );
    }
}

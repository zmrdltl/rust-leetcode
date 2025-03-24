/// Jewels and Stones
///
/// # Arguments
///
/// * `jewels` - A string slice containing the types of jewels
/// * `stones` - A string slice containing the types of stones
///
/// # Returns
///
/// * `Result<i32, &'static str>` - The number of jewels in the stones or an error if the addition would cause an integer overflow.
///
/// # Errors
///
/// Returns an error if jewels or stones are not between 1 and 50 characters.
///
/// # Examples
///
/// ```
/// use rust_leetcode::strings::num_jewels_in_stones;
///
/// assert_eq!(num_jewels_in_stones("aA", "aAAbbbb"), Ok(3));
/// ```
pub fn num_jewels_in_stones(jewels: &str, stones: &str) -> Result<usize, &'static str> {
    if jewels.is_empty() || jewels.len() > 50 || stones.is_empty() || stones.len() > 50 {
        return Err("Jewels or stones are not between 1 and 50 characters");
    }

    Ok(stones.chars().filter(|c| jewels.contains(*c)).count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        let jewels = "aA";
        let stones = "aAAbbbb";
        assert_eq!(num_jewels_in_stones(jewels, stones), Ok(3));
    }

    #[test]
    fn test_edge_cases() {
        let jewels = "z";
        let stones = "";
        assert_eq!(
            num_jewels_in_stones(jewels, stones),
            Err("Jewels or stones are not between 1 and 50 characters")
        );

        let jewels = "";
        let stones = "z";
        assert_eq!(
            num_jewels_in_stones(jewels, stones),
            Err("Jewels or stones are not between 1 and 50 characters")
        );
    }
}

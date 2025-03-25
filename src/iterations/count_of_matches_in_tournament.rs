/// Count Matches in Tournament
///
/// # Arguments
///
/// * `n` - Number of teams
///
/// # Returns
///
/// * `Result<i32, &'static str>` - The number of matches played in the tournament, or an error if the input is invalid
///
/// # Errors
///
/// Returns an error if the input is invalid.
///
/// # Examples
///
/// ```
/// use rust_leetcode::iterations::number_of_matches;
///
/// assert_eq!(number_of_matches(7), Ok(6));
/// ```
pub fn number_of_matches(n: i32) -> Result<i32, &'static str> {
    if !(1..=200).contains(&n) {
        return Err("n must be between 1 and 200");
    }

    Ok(n - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        assert_eq!(number_of_matches(7), Ok(6));
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(number_of_matches(0), Err("n must be between 1 and 200"));
    }
}

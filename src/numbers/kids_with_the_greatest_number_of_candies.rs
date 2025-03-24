/// Kids with the greatest number of candies
///
/// # Arguments
///
/// * `candies` - A vector of i32 values representing the number of candies each kid has
/// * `extra_candies` - A i32 value representing the number of extra candies to give to each kid
///
/// # Returns
///
/// * `Result<Vec<bool>, &'static str>` - A vector of booleans indicating whether each kid has the greatest number of candies or not
///
/// # Errors
///
/// Returns an error if:
/// - The input slice is less than 2 elements
/// - The input extra candies is not between 1 and 50
///
/// # Panics
///
/// This function should not panic as all error conditions are handled with Results.
///
/// # Examples
///
/// ```
/// # use rust_leetcode::numbers::kids_with_candies;
/// let candies = vec![2, 3, 5, 1, 3];
/// let extra_candies = 3;
/// assert_eq!(kids_with_candies(&candies, extra_candies), Ok(vec![true, true, true, false, true]));
/// ```
pub fn kids_with_candies(candies: &[i32], extra_candies: i32) -> Result<Vec<bool>, &'static str> {
    if candies.len() < 2 {
        return Err("At least 2 kids are required");
    }
    if !(1..=50).contains(&extra_candies) {
        return Err("Extra candies must be between 1 and 50");
    }

    let max_candies = candies
        .iter()
        .max()
        .expect("Array already validated to have at least 2 elements, so max must exist");

    Ok(candies
        .iter()
        .map(|&c| c + extra_candies >= *max_candies)
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        let candies = vec![2, 3, 5, 1, 3];
        let extra_candies = 3;
        assert_eq!(
            kids_with_candies(&candies, extra_candies),
            Ok(vec![true, true, true, false, true])
        );
    }

    #[test]
    fn test_edge_cases() {
        let candies = vec![];
        let extra_candies = 3;
        assert_eq!(
            kids_with_candies(&candies, extra_candies),
            Err("At least 2 kids are required")
        );

        let candies = vec![1, 2];
        let extra_candies = 0;
        assert_eq!(
            kids_with_candies(&candies, extra_candies),
            Err("Extra candies must be between 1 and 50")
        );
    }
}

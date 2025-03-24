/// How many numbers are smaller than the current number
///
/// Given an array of integers `nums`, this function returns an array of integers
/// where each element `ans[i]` is the number of elements in `nums` that are smaller than `nums[i]`.
///
/// # Arguments
///
/// * `nums` - A slice of integers
///
/// # Returns
///
/// * `Result<Vec<i32>, &'static str>` - A vector of counts or an error message
///
/// # Errors
///
/// Returns an error if:
/// - nums.length is not between 2 and 500
/// - nums elements are not between 0 and 100
/// - Internal computation errors occur
///
/// # Examples
///
/// ```
/// use rust_leetcode::numbers::smaller_numbers_than_current;
///
/// let nums = vec![8, 1, 2, 2, 3];
/// assert_eq!(smaller_numbers_than_current(&nums), Ok(vec![4, 0, 1, 1, 3]));
/// ```
pub fn smaller_numbers_than_current(nums: &[i32]) -> Result<Vec<usize>, &'static str> {
    if nums.len() < 2 || nums.len() > 500 {
        return Err("nums.length is not between 2 and 500");
    }

    if nums.iter().any(|&num| !(0..=100).contains(&num)) {
        return Err("nums elements are not between 0 and 100");
    }

    Ok(nums
        .iter()
        .map(|&current| nums.iter().filter(|&&num| num < current).count())
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        assert_eq!(
            smaller_numbers_than_current(&[8, 1, 2, 2, 3]),
            Ok(vec![4, 0, 1, 1, 3])
        );
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(
            smaller_numbers_than_current(&[1]),
            Err("nums.length is not between 2 and 500")
        );

        assert_eq!(
            smaller_numbers_than_current(&[1, 1, 101]),
            Err("nums elements are not between 0 and 100")
        );
    }
}

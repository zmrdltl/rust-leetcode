/// Build array from permutation.
///
/// Given an array of integers `nums` of length n, this function returns
/// a new array `ans` of length n where `ans[i] == nums[nums[i]]` for 0 <= i < n.
///
/// # Arguments
///
/// * `nums` - A slice of integers to build array from permutation
///
/// # Returns
///
/// * `Vec<i32>` - The built array
///
/// # Errors
///
/// Returns an error if:
/// - The input slice is empty
///
/// # Examples
///
/// ```
/// use rust_leetcode::vectors::build_array_from_permutation;
///
/// let nums = vec![0, 2, 1, 5, 3, 4];
/// assert_eq!(build_array_from_permutation(&nums), vec![0, 1, 2, 4, 5, 3]);
/// ```
#[must_use]
pub fn build_array_from_permutation(nums: &[usize]) -> Vec<i32> {
    nums.iter()
        .filter_map(|&num| {
            if num < nums.len() {
                nums.get(num).and_then(|&val| i32::try_from(val).ok())
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        let nums = vec![0, 2, 1, 5, 3, 4];
        let result = build_array_from_permutation(&nums);
        assert_eq!(result, vec![0, 1, 2, 4, 5, 3]);
    }

    #[test]
    fn test_edge_cases() {
        let nums = vec![];
        let result = build_array_from_permutation(&nums);
        assert_eq!(result, vec![]);
    }
}

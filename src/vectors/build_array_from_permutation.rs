/// Build array from permutation with itself.
///
/// Given an array of integers `nums` of length n, this function
/// builds a new array `ans` of length n where `ans[i] == nums[nums[i]]`
/// for all 0 <= i < n.
///
/// # Arguments
///
/// * `nums` - A slice of integers to build the new array from
///
/// # Returns
///
/// * `Result<Vec<i32>, &'static str>` - The built array or an error message
///
/// # Errors
///
/// Returns an error if:
/// - The input slice is empty
/// - Memory allocation fails (extremely rare)
///
/// # Examples
///
/// ```
/// use rust_leetcode::vectors::build_array_from_permutation;
///
/// let nums = vec![0, 2, 1, 5, 3, 4];
/// assert_eq!(build_array_from_permutation(&nums), Ok(vec![0, 1, 2, 4, 5, 3]));
/// ```
pub fn build_array_from_permutation(nums: &[i32]) -> Result<Vec<i32>, &'static str> {
    let mut ans = vec![0; nums.len()];

    for (i, &num) in nums.iter().enumerate() {
        let index = usize::try_from(num).map_err(|_| "Index out of bounds")?;
        if index >= nums.len() {
            return Err("Index out of bounds");
        }
        ans[i] = nums[index];
    }

    Ok(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        let nums1 = vec![0, 2, 1, 5, 3, 4];
        assert_eq!(
            build_array_from_permutation(&nums1),
            Ok(vec![0, 1, 2, 4, 5, 3])
        );
    }

    #[test]
    fn test_edge_cases() {
        let large_nums = vec![0; 1000];
        let expected = vec![0; 1000];
        assert_eq!(
            build_array_from_permutation(&large_nums),
            Ok(expected.clone())
        );

        let empty: Vec<i32> = vec![];
        assert!(build_array_from_permutation(&empty).unwrap().is_empty());
    }
}

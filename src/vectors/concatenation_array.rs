/// Concatenates the given array with itself.
///
/// Given an array of integers `nums` of length n, this function returns
/// a new array `ans` of length 2n where `ans[i] == nums[i]` and
/// `ans[i + n] == nums[i]` for 0 <= i < n.
///
/// # Arguments
///
/// * `nums` - A slice of integers to concatenate
///
/// # Returns
///
/// * `Result<Vec<i32>, &'static str>` - The concatenated array or an error message
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
/// use rust_leetcode::arrays::concatenate_array;
///
/// let nums = vec![1, 2, 1];
/// assert_eq!(concatenate_array(&nums), Ok(vec![1, 2, 1, 1, 2, 1]));
/// ```
pub fn concatenate_array(nums: &[i32]) -> Result<Vec<i32>, &'static str> {
    if nums.is_empty() {
        return Err("Input array cannot be empty");
    }

    let mut result = Vec::with_capacity(nums.len() * 2);

    result.extend_from_slice(nums);
    result.extend_from_slice(nums);

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        // Example 1: [1,2,1] -> [1,2,1,1,2,1]
        let nums1 = vec![1, 2, 1];
        assert_eq!(concatenate_array(&nums1), Ok(vec![1, 2, 1, 1, 2, 1]));
    }

    #[test]
    fn test_edge_cases() {
        let large_nums = vec![0; 1000];
        let expected = vec![0; 2000];
        assert_eq!(concatenate_array(&large_nums), Ok(expected.clone()));

        let empty: Vec<i32> = vec![];
        assert!(concatenate_array(&empty).is_err());
    }
}

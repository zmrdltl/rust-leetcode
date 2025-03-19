/// Shuffle the Array
///
/// Given an array of integers `nums` of length n, this function
/// returns an array `ans` of length n where `ans[i] == nums[0] + nums[1] + ... + nums[i]`
///
/// # Arguments
///
/// * `nums` - A slice of integers to build the new array from
///
/// # Returns
///
/// * `Result<Vec<i32>, &'static str>` - The shuffled array
///
/// # Errors
///
/// Returns an error if:
/// - The input slice is empty
/// - Memory allocation fails
///
/// # Examples
///
/// ```
/// use rust_leetcode::vectors::shuffle_the_array;
///
/// let nums = vec![2, 5, 1, 3, 4, 7];
/// let n = 3;
/// assert_eq!(shuffle_the_array(&nums, n), Ok(vec![2, 3, 5, 4, 1, 7]));
/// ```
pub fn shuffle_the_array(nums: &[i32], n: i32) -> Result<Vec<i32>, &'static str> {
    if nums.is_empty() {
        return Err("Input array is empty");
    }
    if nums.len() > 500 {
        return Err("Input array length exceeds maximum allowed");
    }
    let n = usize::try_from(n).map_err(|_| "Index out of bounds")?;

    if nums.len() != 2 * n {
        return Err("Input array length must be twice the value of n");
    }

    let mut ans = vec![0; nums.len()];
    (0..n).for_each(|i| {
        ans[2 * i] = nums[i];
        ans[2 * i + 1] = nums[n + i];
    });

    Ok(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        let nums = vec![2, 5, 1, 3, 4, 7];
        let n = 3;
        assert_eq!(shuffle_the_array(&nums, n), Ok(vec![2, 3, 5, 4, 1, 7]));
    }

    #[test]
    fn test_edge_cases() {
        let nums = vec![1];
        let n = 2;
        assert_eq!(
            shuffle_the_array(&nums, n),
            Err("Input array length must be twice the value of n")
        );

        let nums = vec![0; 501];
        let n = 1;
        assert_eq!(
            shuffle_the_array(&nums, n),
            Err("Input array length exceeds maximum allowed")
        );

        let nums = vec![];
        let n = 0;
        assert_eq!(shuffle_the_array(&nums, n), Err("Input array is empty"));
    }
}

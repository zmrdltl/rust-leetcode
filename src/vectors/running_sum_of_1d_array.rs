/// Running Sum of 1d Array
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
/// * `Result<Vec<i32>, &'static str>` - The running sum of the array
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
/// use rust_leetcode::vectors::running_sum_of_1d_array;
///
/// let nums = vec![1, 2, 3, 4];
/// assert_eq!(running_sum_of_1d_array(&nums), Ok(vec![1, 3, 6, 10]));
/// ```
pub fn running_sum_of_1d_array(nums: &[i32]) -> Result<Vec<i32>, &'static str> {
    if nums.len() > 1000 {
        return Err("Input array length exceeds maximum allowed");
    }

    let mut ans = vec![0; nums.len()];
    let mut sum = 0;
    for (i, &num) in nums.iter().enumerate() {
        sum += num;
        ans[i] = sum;
    }

    Ok(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(running_sum_of_1d_array(&nums), Ok(vec![1, 3, 6, 10]));
    }

    #[test]
    fn test_edge_cases() {
        let empty: Vec<i32> = vec![];
        assert!(running_sum_of_1d_array(&empty).unwrap().is_empty());

        let large_nums = vec![0; 1001];
        assert_eq!(
            running_sum_of_1d_array(&large_nums),
            Err("Input array length exceeds maximum allowed")
        );
    }
}

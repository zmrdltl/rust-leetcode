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
pub fn smaller_numbers_than_current(nums: &[i32]) -> Result<Vec<i32>, &'static str> {
    if nums.len() < 2 || nums.len() > 500 {
        return Err("nums.length is not between 2 and 500");
    }

    // Validate all elements are within the expected range
    if nums.iter().any(|&num| !(0..=100).contains(&num)) {
        return Err("nums elements are not between 0 and 100");
    }

    let frequency: [i32; 101] = nums.iter().fold([0; 101], |mut acc, &num| {
        if let Ok(idx) = usize::try_from(num) {
            if idx <= 100 {
                acc[idx] += 1;
            }
        }
        acc
    });

    let counts: [i32; 101] = (0..101).fold([0; 101], |mut acc, i| {
        if i > 0 {
            acc[i] = acc[i - 1] + frequency[i - 1];
        }
        acc
    });

    // Map each number to its smaller count
    nums.iter()
        .map(|&num| {
            usize::try_from(num)
                .map_err(|_| "Invalid number conversion")
                .and_then(|idx| {
                    if idx <= 100 {
                        Ok(counts[idx])
                    } else {
                        Err("Number out of valid range")
                    }
                })
        })
        .collect()
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

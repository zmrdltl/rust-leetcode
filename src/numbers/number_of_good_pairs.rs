use std::collections::HashMap;

/// Calculates the number of good pairs in the given array
/// A pair (i, j) is called good if nums\[i\] == nums\[j\] and i < j.
///
/// # Arguments
///
/// * `nums` - A slice of usize values to check for good pairs
///
/// # Returns
///
/// * `Result<i32, &'static str>` - The count of good pairs or an error message
///
/// # Errors
///
/// Returns an error if:
/// - The input slice is empty
/// - The input slice length exceeds 100
///
/// # Examples
///
/// ```
/// use rust_leetcode::numbers::number_of_good_pairs;
///
/// let nums = vec![1, 2, 3, 1, 1, 3];
/// assert_eq!(number_of_good_pairs(&nums), Ok(4));
/// ```
pub fn number_of_good_pairs(nums: &[usize]) -> Result<i32, &'static str> {
    if nums.is_empty() {
        return Err("Input array cannot be empty");
    }
    if nums.len() > 100 {
        return Err("Input array length exceeds maximum allowed");
    }

    let counts = nums.iter().fold(HashMap::new(), |mut counts, &num| {
        *counts.entry(num).or_insert(0) += 1;
        counts
    });

    let pairs = counts
        .values()
        .filter(|&&count| count > 1)
        .map(|&count| count * (count - 1) / 2)
        .sum::<i32>();

    Ok(pairs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        let nums1 = [1, 2, 3, 1, 1, 3];
        assert_eq!(number_of_good_pairs(&nums1), Ok(4));

        let nums2 = [1, 1, 1, 1];
        assert_eq!(number_of_good_pairs(&nums2), Ok(6));

        let nums3 = [1, 2, 3];
        assert_eq!(number_of_good_pairs(&nums3), Ok(0));
    }

    #[test]
    fn test_empty_input() {
        let empty = [];
        assert_eq!(
            number_of_good_pairs(&empty),
            Err("Input array cannot be empty")
        );
    }

    #[test]
    fn test_large_numbers() {
        let large_array = vec![0; 101];
        assert_eq!(
            number_of_good_pairs(&large_array),
            Err("Input array length exceeds maximum allowed")
        );
    }
}

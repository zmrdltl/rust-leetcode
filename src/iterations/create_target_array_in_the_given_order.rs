/// Create Target Array in the Given Order
///
/// # Arguments
///
/// * `nums` - The array of integers
/// * `index` - The array of indices
///
/// # Returns
///
/// * `Result<Vec<i32>, &'static str>` - The target array, or an error if the input is invalid
///
/// # Errors
///
/// Returns an error if the input is invalid.
///
/// # Examples
///
/// ```
/// use rust_leetcode::iterations::create_target_array;
///
/// assert_eq!(
///     create_target_array(&[0, 1, 2, 3, 4], &[0, 1, 2, 2, 1]),
///     Ok(vec![0, 4, 1, 3, 2])
/// );
/// ```
pub fn create_target_array(nums: &[usize], index: &[usize]) -> Result<Vec<usize>, &'static str> {
    if nums.len() != index.len() {
        return Err("nums and index must have the same length");
    }

    if nums.is_empty() || nums.len() > 100 || index.is_empty() || index.len() > 100 {
        return Err("Length of nums and index must be between 1 and 100");
    }

    if nums.iter().any(|&n| !(0..=100).contains(&n))
        || index.iter().any(|&i| !(0..=100).contains(&i))
    {
        return Err("Elements of nums and index must be between 0 and 100");
    }

    Ok(index.iter().zip(nums.iter()).fold(
        Vec::with_capacity(nums.len()),
        |mut acc, (&idx, &num)| {
            acc.insert(idx, num);
            acc
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        assert_eq!(
            create_target_array(&[0, 1, 2, 3, 4], &[0, 1, 2, 2, 1]),
            Ok(vec![0, 4, 1, 3, 2])
        );
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(
            create_target_array(&[1], &[1, 2]),
            Err("nums and index must have the same length")
        );

        assert_eq!(
            create_target_array(&[], &[]),
            Err("Length of nums and index must be between 1 and 100")
        );

        assert_eq!(
            create_target_array(&[1], &[101]),
            Err("Elements of nums and index must be between 0 and 100")
        );
    }
}

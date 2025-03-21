/// Minimum sum of four digit number after splitting digits
///
/// Given a four digit number `num`, this function returns the minimum sum of two numbers
/// formed by splitting the digits of `num` into two new numbers.
///
/// # Arguments
///
/// * `num` - A four digit number
///
/// # Returns
///
/// * `Result<i32, &'static str>` - The minimum sum of two numbers or an error message
///
/// # Errors
///
/// Returns an error if:
/// - The input is not a four digit number
///
/// # Examples
///
/// ```
/// use rust_leetcode::numbers::minimum_sum;
///
/// let num = 2932;
/// assert_eq!(minimum_sum(num), Ok(52));
/// ```
pub fn minimum_sum(num: i32) -> Result<i32, &'static str> {
    if !(1000..=9999).contains(&num) {
        return Err("Input is not a four digit number");
    }

    let mut digits: Vec<i32> = num
        .to_string()
        .chars()
        .filter_map(|c| c.to_digit(10).and_then(|d| i32::try_from(d).ok()))
        .collect();

    digits.sort_unstable();

    Ok((digits[0] * 10 + digits[2]) + (digits[1] * 10 + digits[3]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        assert_eq!(minimum_sum(2932), Ok(52));
        assert_eq!(minimum_sum(4009), Ok(13));
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(minimum_sum(1), Err("Input is not a four digit number"));
    }
}

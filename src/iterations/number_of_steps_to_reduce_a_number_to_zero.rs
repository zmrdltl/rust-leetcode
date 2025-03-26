use std::iter::successors;

/// Number of steps to reduce a number to zero
///
/// # Arguments
///
/// * `num` - The number to reduce
///
/// # Returns
///
/// * `Result<i32, &'static str>` - The number of steps to reduce the number to zero
///
/// # Errors
///
/// Returns an error if the number is not between 0 and 10^6.
///
/// # Examples
///
/// ```
/// use rust_leetcode::iterations::number_of_steps;
///
/// assert_eq!(number_of_steps(8), Ok(4));
/// ```
pub fn number_of_steps(num: i32) -> Result<usize, &'static str> {
    if !(0..=10_000_000).contains(&num) {
        return Err("Number is not between 0 and 10^6");
    }

    Ok(successors(Some(num), |&n| {
        if n > 0 {
            Some(if n % 2 == 0 { n / 2 } else { n - 1 })
        } else {
            None
        }
    })
    .count()
        - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        assert_eq!(number_of_steps(8), Ok(4));
        assert_eq!(number_of_steps(14), Ok(6));
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(
            number_of_steps(10_000_001),
            Err("Number is not between 0 and 10^6")
        );
    }
}

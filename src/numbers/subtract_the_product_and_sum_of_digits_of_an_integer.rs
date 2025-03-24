/// Subtract the product and sum of digits of an integer.
///
/// Given an integer `n`, this function returns the difference between the product of its digits and the sum of its digits.
///
/// # Arguments
///
/// * `n` - An integer
///
/// # Returns
///
/// * `i32` - The difference between the product of its digits and the sum of its digits
///
/// # Examples
///
/// ```
/// use rust_leetcode::numbers::subtract_product_and_sum;
///
/// let n = 234;
/// assert_eq!(subtract_product_and_sum(n), 15);
/// ```
///
/// ```
/// use rust_leetcode::numbers::subtract_product_and_sum;
///
/// let n = 4421;
/// assert_eq!(subtract_product_and_sum(n), 21);
/// ```
#[must_use]
pub fn subtract_product_and_sum(n: i32) -> i32 {
    let mut product = 1;
    let mut sum = 0;
    let mut n = n;
    while n > 0 {
        let digit = n % 10;
        product *= digit;
        sum += digit;
        n /= 10;
    }
    product - sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        assert_eq!(subtract_product_and_sum(234), 15);
        assert_eq!(subtract_product_and_sum(4421), 21);
    }
}

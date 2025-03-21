/// Running Richest Customer Wealth
///
/// Given an array of integers `accounts` of length `m` where each `accounts[i]` is an array of
/// integers of length `n` and `accounts[i][j]` represents the amount of money the `i-th` customer
/// has in the `j-th` bank, this function returns the wealth of the richest customer.
///
/// # Arguments
///
/// * `accounts` - A slice of slices of integers representing the wealth of each customer
///
/// # Returns
///
/// * `Result<i32, &'static str>` - The wealth of the richest customer
///
/// # Errors
///
/// Returns an error if:
/// - The input slice is empty
/// - The input slice is not a valid array
///
/// # Examples
///
/// ```
/// use rust_leetcode::vectors::richest_customer_wealth;
///
/// let accounts = vec![vec![1, 2, 3], vec![3, 2, 1]];
/// assert_eq!(richest_customer_wealth(&accounts), Ok(6));
/// ```
pub fn richest_customer_wealth(accounts: &[Vec<i32>]) -> Result<i32, &'static str> {
    if accounts.is_empty() {
        return Err("Input array cannot be empty");
    }

    if accounts.iter().any(Vec::is_empty) {
        return Err("Input array cannot contain empty arrays");
    }

    accounts
        .iter()
        .map(|account| account.iter().sum())
        .max()
        .ok_or("Failed to calculate maximum wealth")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        let accounts = vec![vec![1, 2, 3], vec![3, 2, 1]];
        assert_eq!(richest_customer_wealth(&accounts), Ok(6));
    }

    #[test]
    fn test_edge_cases() {
        let accounts = vec![];
        assert_eq!(
            richest_customer_wealth(&accounts),
            Err("Input array cannot be empty")
        );

        let accounts = vec![vec![], vec![1]];
        assert_eq!(
            richest_customer_wealth(&accounts),
            Err("Input array cannot contain empty arrays")
        );
    }
}

/// Count items matching a rule
///
/// # Arguments
///
/// * `items` - A list of items
/// * `rule_key` - The key to match
/// * `rule_value` - The value to match
///
/// # Returns
///
/// * `Result<i32, &'static str>` - The number of items matching the rule
///
/// # Errors
///
/// Returns an error if the addition would cause an integer overflow.
/// Returns an error if items length is not between 1 and 10000.
/// Returns an error if `rule_key` is not "type", "color", or "name".
/// Returns an error if any item doesn't have enough elements (at least 3).
///
///
/// # Examples
///
/// ```
/// use rust_leetcode::strings::count_matches;
///
/// let items = vec![
///     vec!["phone", "blue", "pixel"],
///     vec!["computer", "silver", "lenovo"],
///     vec!["phone", "gold", "iphone"]
/// ];
/// assert_eq!(count_matches(&items, "color", "silver"), Ok(1));
/// ```
pub fn count_matches(
    items: &[Vec<&str>],
    rule_key: &str,
    rule_value: &str,
) -> Result<usize, &'static str> {
    if items.is_empty() || items.len() > 10000 {
        return Err("items length must be between 1 and 10000");
    }

    let rule_key_index = match rule_key {
        "type" => 0,
        "color" => 1,
        "name" => 2,
        _ => return Err("Invalid rule key"),
    };

    if items.iter().any(|item| item.len() <= rule_key_index) {
        return Err("Item doesn't have enough elements");
    }

    Ok(items
        .iter()
        .filter(|item| item[rule_key_index] == rule_value)
        .count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        let items = vec![
            vec!["phone", "blue", "pixel"],
            vec!["computer", "silver", "lenovo"],
            vec!["phone", "gold", "iphone"],
        ];
        assert_eq!(count_matches(&items, "color", "silver"), Ok(1));
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(
            count_matches(&[], "color", "silver"),
            Err("items length must be between 1 and 10000")
        );
        assert_eq!(
            count_matches(&[vec!["phone", "blue", "pixel"]], "hi", "silver"),
            Err("Invalid rule key")
        );

        assert_eq!(
            count_matches(&[vec!["phone", "blue"]], "name", "pixel"),
            Err("Item doesn't have enough elements")
        );
    }
}

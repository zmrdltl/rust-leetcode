/// Maximum number of words found in sentences
///
/// # Arguments
///
/// * `sentences` - A vector of sentences
///
/// # Returns
///
/// * `Result<i32, &'static str>` - The maximum number of words found in sentences
///
/// # Errors
///
/// Returns an error if the sentences are not valid.
///
/// # Panics
///
/// This function should not panic as all error conditions are handled with Results.
///
/// # Examples
///
/// ```
/// use rust_leetcode::strings::most_words_found;
///
/// assert_eq!(most_words_found(&["hello world", "programming is fun"]), Ok(3));
/// ```
pub fn most_words_found(sentences: &[&str]) -> Result<usize, &'static str> {
    if sentences.is_empty() || sentences.len() > 100 {
        return Err("Input array cannot be empty or exceed 100 elements");
    }

    Ok(sentences
        .iter()
        .map(|sentence| sentence.split_whitespace().count())
        .max()
        .expect("cannot make panic here because sentences are not empty"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        assert_eq!(
            most_words_found(&[
                "alice and bob love leetcode",
                "i think so too",
                "this is great thanks very much"
            ]),
            Ok(6)
        );
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(
            most_words_found(&[]),
            Err("Input array cannot be empty or exceed 100 elements")
        );
    }
}

/// Sorting the Sentence
///
/// # Arguments
///
/// * `s` - A string
///
/// # Returns
///
/// * `Result<String, &'static str>` - The sorted sentence, or an error if the input is invalid
///
/// # Errors
///
/// Returns an error if the input is invalid.
///
/// # Examples
///
/// ```
/// use rust_leetcode::strings::sort_sentence;
///
/// assert_eq!(sort_sentence("is2 sentence4 This1 a3"), Ok("This is a sentence".to_string()));
/// ```
pub fn sort_sentence(s: &str) -> Result<String, &'static str> {
    if s.len() < 2 || s.len() > 200 {
        return Err("Input string must be between 2 and 200 characters");
    }

    let mut words_with_positions = s
        .split_whitespace()
        .filter_map(|word| {
            if word.is_empty() || !word.chars().last()?.is_ascii_digit() {
                return Some(Err("Word without valid position number"));
            }

            let (text, pos_str) = word.split_at(word.len() - 1);
            let position = pos_str.parse::<u32>().ok()?;

            Some(Ok((text, position)))
        })
        .collect::<Result<Vec<_>, &'static str>>()?;

    words_with_positions.sort_unstable_by_key(|(_, pos)| *pos);

    Ok(words_with_positions
        .into_iter()
        .map(|(text, _)| text)
        .collect::<Vec<_>>()
        .join(" "))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        assert_eq!(
            sort_sentence("is2 sentence4 This1 a3"),
            Ok("This is a sentence".to_string())
        );
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(
            sort_sentence(""),
            Err("Input string must be between 2 and 200 characters")
        );

        assert_eq!(
            sort_sentence("Hi"),
            Err("Word without valid position number")
        );
    }
}

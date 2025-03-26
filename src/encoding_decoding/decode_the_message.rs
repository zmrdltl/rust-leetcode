use std::collections::HashMap;

/// Decode the Message
///
/// # Arguments
///
/// * `key` - The key to decode the message
/// * `message` - The message to decode
///
/// # Returns
///
/// * `Result<String, &'static str>` - The decoded message, or an error if the key is not valid
///
/// # Errors
///
/// Returns an error if the key is not valid.
///
/// # Examples
///
/// ```
/// use rust_leetcode::encoding_decoding::decode_message;
///
/// assert_eq!(decode_message("the quick brown fox jumps over the lazy dog", "vkbs bs t suepuv"), Ok("this is a secret".to_string()));
/// ```
pub fn decode_message(key: &str, message: &str) -> Result<String, &'static str> {
    if key.len() < 26 || key.len() > 2000 {
        return Err("Key must be between 26 and 2000 characters");
    }

    if message.is_empty() || message.len() > 2000 {
        return Err("Message must be between 1 and 2000 characters");
    }

    if key
        .chars()
        .any(|c| c != ' ' && (!c.is_ascii_alphabetic() || !c.is_ascii_lowercase()))
    {
        return Err("Key must contain only lowercase letters");
    }

    if message
        .chars()
        .any(|c| c != ' ' && (!c.is_ascii_alphabetic() || !c.is_ascii_lowercase()))
    {
        return Err("Message must contain only lowercase letters");
    }

    let mut key_chars_map = HashMap::new();
    let alphabet: Vec<char> = ('a'..='z').collect();

    let mut index = 0;
    for c in key.chars().filter(|&c| c != ' ') {
        if index < alphabet.len() && !key_chars_map.contains_key(&c) {
            key_chars_map.insert(c, alphabet[index]);
            index += 1;
        }
    }

    let decoded_message = message
        .chars()
        .map(|c| *key_chars_map.get(&c).unwrap_or(&c))
        .collect();

    Ok(decoded_message)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        assert_eq!(
            decode_message(
                "the quick brown fox jumps over the lazy dog",
                "vkbs bs t suepuv"
            ),
            Ok("this is a secret".to_string())
        );
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(
            decode_message("a", "b"),
            Err("Key must be between 26 and 2000 characters")
        );

        assert_eq!(
            decode_message("the quick brown fox jumps over the lazy dog", ""),
            Err("Message must be between 1 and 2000 characters")
        );

        assert_eq!(
            decode_message("the quick brown fox jumps over the lazy dog@", "b"),
            Err("Key must contain only lowercase letters")
        );

        assert_eq!(
            decode_message("the quick brown fox jumps over the lazy dog", "@"),
            Err("Message must contain only lowercase letters")
        );
    }
}

/// Converts a single word to alternating case (lowercase at even indices, uppercase at odd indices)
pub fn alt_case_word(subject: &str) -> String {
    subject
        .chars()
        .enumerate()
        .map(|(idx, letter)| {
            if idx % 2 == 0 {
                letter.to_lowercase().to_string()
            } else {
                letter.to_uppercase().to_string()
            }
        })
        .collect()
}

/// Converts a string to mixed case, processing each word separately
/// 
/// # Arguments
/// 
/// * `subject` - The input string to convert
/// 
/// # Returns
/// 
/// Returns the mixed case string, or an error if subject is None
pub fn alt_case_str(subject: &str) -> Result<String, String> {
    if subject.is_empty() {
        return Ok(String::new());
    }
    
    Ok(subject
        .split(' ')
        .map(alt_case_word)
        .collect::<Vec<String>>()
        .join(" "))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alt_case_word_converts_single_word_to_alternating_case() {
        assert_eq!(alt_case_word("hello"), "hElLo");
        assert_eq!(alt_case_word("WORLD"), "wOrLd");
        assert_eq!(alt_case_word("MiXeD"), "mIxEd");
    }

    #[test]
    fn test_alt_case_word_handles_empty_string() {
        assert_eq!(alt_case_word(""), "");
    }

    #[test]
    fn test_alt_case_word_handles_single_character() {
        assert_eq!(alt_case_word("a"), "a");
        assert_eq!(alt_case_word("A"), "a");
    }

    #[test]
    fn test_alt_case_word_preserves_non_alphabetic_characters() {
        assert_eq!(alt_case_word("hello123"), "hElLo123");
        assert_eq!(alt_case_word("!@#$%"), "!@#$%");
    }

    #[test]
    fn test_alt_case_str_converts_multiple_words_to_alternating_case() {
        assert_eq!(alt_case_str("hello world").unwrap(), "hElLo wOrLd");
        assert_eq!(alt_case_str("THIS IS A TEST").unwrap(), "tHiS iS a tEsT");
    }

    #[test]
    fn test_alt_case_str_handles_empty_string() {
        assert_eq!(alt_case_str("").unwrap(), "");
    }

    #[test]
    fn test_alt_case_str_handles_single_word() {
        assert_eq!(alt_case_str("hello").unwrap(), "hElLo");
    }

    #[test]
    fn test_alt_case_str_handles_multiple_spaces() {
        assert_eq!(alt_case_str("hello   world").unwrap(), "hElLo   wOrLd");
    }

    #[test]
    fn test_alt_case_str_preserves_punctuation_and_special_characters() {
        assert_eq!(alt_case_str("hello, world!").unwrap(), "hElLo, wOrLd!");
        assert_eq!(alt_case_str("test@example.com").unwrap(), "tEsT@ExAmPlE.CoM");
    }
}

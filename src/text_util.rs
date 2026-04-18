
use deunicode::deunicode;
use regex::Regex;
use once_cell::sync::Lazy;

// We split on punctuation followed by a space, keeping the punctuation
static SENTENCE_DELIMS: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?<=[.!?])\s+").unwrap());
// Remove spaces that occur before punctuation marks
static PUNCTUATION_GAP: Lazy<Regex> = Lazy::new(||  Regex::new(r"\s+([.,!?;:])").unwrap());



/// Sanitizes a string by normalizing Unicode, collapsing whitespace, and fixing punctuation spacing.
///
/// This function performs three main cleanup steps:
/// 1. **De-unicoding**: Converts non-ASCII characters to their closest ASCII equivalents (e.g., 'é' -> 'e').
/// 2. **Whitespace Collapse**: Replaces multiple consecutive whitespace characters (spaces, tabs, newlines) with a single space.
/// 3. **Punctuation Correction**: Removes any whitespace immediately preceding common punctuation marks (`.`, `,`, `!`, `?`, `;`, `:`).
///
/// # Examples
///
/// ```
/// use fenominal::text_util::sanitize;
///
/// let input = "Frequent  emesis , with feeds resulting in failure to thrive (FTT).";
/// let output = sanitize(input);
/// assert_eq!(output, "Frequent emesis, with feeds resulting in failure to thrive (FTT).");
/// ```
pub fn sanitize(input_text: &str) -> String {
    let deunicoded = deunicode(input_text);
    let re_whitespace = Regex::new(r"\s+").unwrap();
    let collapsed = re_whitespace.replace_all(&deunicoded, " ");
    // 3. Remove spaces BEFORE punctuation (e.g., "emesis , " -> "emesis, ")
    // This looks for a space followed by any character in the set [.,!?;:]
   
    let fixed_punctuation = PUNCTUATION_GAP.replace_all(&collapsed, "$1");
    let cleaned_text = fixed_punctuation.trim();

    return cleaned_text.to_string();
}


pub fn sentence_split(input_text: &str) -> Vec<String> {
    let sentences: Vec<&str> = SENTENCE_DELIMS.split(input_text).collect();
    return sentences.iter().map(|s| s.to_string()).collect();
}



#[cfg(test)]
mod test {
    use std::assert_eq;
    use rstest::rstest;
    use super::*;


    #[rstest]
    // Each case follows the pattern: case(input, expected_output)
    #[case(
        "An 8-week-old female twin born at 36 weeks had a history of frequent  diarrhea  and  emesis , with feeds resulting in failure to thrive (FTT).",
        "An 8-week-old female twin born at 36 weeks had a history of frequent diarrhea and emesis, with feeds resulting in failure to thrive (FTT)."
    )]
    #[case(
        "Hello  world ! ", 
        "Hello world!"
    )]
    #[case(
        "Testing tabs\tand\nnewlines  . ", 
        "Testing tabs and newlines."
    )]
    #[case(
        "Unicode check: résumé  , and café .", 
        "Unicode check: resume, and cafe."
    )]
    fn test_sanitize_cases(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(sanitize(input), expected);
    }

    





}
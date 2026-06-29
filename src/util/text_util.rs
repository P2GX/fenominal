
use deunicode::deunicode;
use regex::Regex;
use once_cell::sync::Lazy;

// We split on punctuation followed by a space, keeping the punctuation
static SENTENCE_DELIMSOLD: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?<=[.!?])\s+").unwrap());
static SENTENCE_DELIMS: Lazy<Regex> = Lazy::new(|| Regex::new(r"([.!?])\s+").unwrap());

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
/// use fenominal::sanitize;
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
    let mut sentences = Vec::new();
    let mut last_end = 0;
    for caps in SENTENCE_DELIMS.captures_iter(input_text) {
        let whole_match = caps.get(0).unwrap();   // punctuation + whitespace
        let punct = caps.get(1).unwrap();         // just the punctuation char

        sentences.push(input_text[last_end..punct.end()].to_string());
        last_end = whole_match.end();
    }

    if last_end < input_text.len() {
        sentences.push(input_text[last_end..].to_string());
    }

    sentences
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
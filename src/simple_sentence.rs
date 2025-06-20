use std::fmt::Display;

use crate::simple_token::SimpleToken;
use once_cell::sync::Lazy;
use regex::Regex;

static WORD_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"[a-zA-Z]+('-[a-zA-Z]+)?").unwrap());

#[derive(Debug, Clone)]
pub struct SimpleSentence {
    sentence: String,
    start_pos: usize,
    end_pos: usize,
    tokens: Vec<SimpleToken>,
}

impl SimpleSentence {
    pub fn new(text: &str, start: usize, end: usize) -> Self {
        let mut stokens = Vec::new();
        for mat in WORD_PATTERN.find_iter(text) {
            stokens.push(SimpleToken::new(
                mat.as_str(),
                mat.as_str(),
                mat.start(),
                mat.end(),
            ));
        }
        SimpleSentence {
            sentence: text.into(),
            start_pos: start,
            end_pos: end,
            tokens: stokens,
        }
    }

    pub fn get_start_pos(&self) -> usize {
        self.start_pos
    }

    pub fn get_end_pos(&self) -> usize {
        self.end_pos
    }

    pub fn get_sentence(&self) -> &str {
        &self.sentence
    }

    pub fn get_tokens(&self) -> &[SimpleToken] {
        &self.tokens
    }
}

impl Display for SimpleSentence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SimpleSentence(id: {} [{}-{}]-# tokens {})",
            self.sentence,
            &self.start_pos,
            &self.end_pos,
            self.tokens.len()
        )
    }
}

#[cfg(test)]
mod test {
    use std::assert_eq;

    use super::*;

    const SENTENCE1: &str = "The quick brown fox jumps over the lazy dog. ";

    #[test]
    fn test_equality() {
        let ssentence = SimpleSentence::new(SENTENCE1, 0, 2);
        assert_eq!(SENTENCE1, ssentence.sentence);
    }

    #[test]
    fn test_tokenize() {
        // sentence 1 has nin tokens
        let ssentence = SimpleSentence::new(SENTENCE1, 0, 2);
        assert_eq!(9, ssentence.tokens.len());
    }

    #[test]
    fn test_equality_of_tokens() {
        let ssentence = SimpleSentence::new(SENTENCE1, 0, 2);
        let tokens = &ssentence.tokens;
        assert_eq!("The", tokens.get(0).unwrap().get_original_token());
        assert_eq!("quick", tokens.get(1).unwrap().get_original_token());
        assert_eq!("brown", tokens.get(2).unwrap().get_original_token());
        assert_eq!("fox", tokens.get(3).unwrap().get_original_token());
        assert_eq!("jumps", tokens.get(4).unwrap().get_original_token());
        assert_eq!("over", tokens.get(5).unwrap().get_original_token());
        assert_eq!("the", tokens.get(6).unwrap().get_original_token());
        assert_eq!("lazy", tokens.get(7).unwrap().get_original_token());
        assert_eq!("dog", tokens.get(8).unwrap().get_original_token());
    }
    #[test]
    fn test_test_positions() {
        let ssentence = SimpleSentence::new(SENTENCE1, 106, 202);
        assert_eq!(106, ssentence.get_start_pos());
        assert_eq!(202, ssentence.get_end_pos());
    }
}

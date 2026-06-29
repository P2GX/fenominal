//! SimpleToken
//! Represents one token (usually, a word) in the original text
//! Includes string and its position and a lower-case version
//!

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SimpleToken {
    token: String,
    original_token: String,
    lowercase_token: String,
    start_pos: usize,
    end_pos: usize,
    pub(crate) index: usize, // Position in the original sentence
}

impl SimpleToken {
    pub fn new<S: Into<String>>(token: S, orig_token: S, start: usize, end: usize, idx: usize) -> Self {
        let o_token_string: String = orig_token.into();
        let lc_token = o_token_string.to_lowercase();
        SimpleToken {
            token: token.into(),
            original_token: o_token_string,
            lowercase_token: lc_token,
            start_pos: start,
            end_pos: end,
            index: idx,
        }
    }

    pub fn get_original_token(&self) -> &str {
        &self.original_token
    }

    pub fn get_lc_original_token(&self) -> &str {
        &self.lowercase_token
    }

    pub fn get_start_pos(&self) -> usize {
        self.start_pos
    }

    pub fn get_end_pos(&self) -> usize {
        self.end_pos
    }

    pub fn length(&self) -> usize {
        1 + self.end_pos - self.start_pos
    }
}

#[cfg(test)]
mod test {
    use std::assert_eq;

    use super::*;

    #[test]
    fn test_lower_case() {
        let tests = vec![("Orange", "orange"), ("Apple", "apple"), ("pear", "pear")];
        let fake_idx = 42 as usize;
        for test in tests {
            let st = SimpleToken::new(test.0, test.0, 1, 2, fake_idx);
            assert_eq!(test.1, st.get_lc_original_token());
        }
    }
}

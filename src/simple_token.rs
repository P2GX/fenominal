

#[derive(Clone)]
pub struct SimpleToken {
    token: String,
    original_token: String,
    start_pos: usize,
    end_pos: usize,
}

impl SimpleToken {


    pub fn new<S: Into<String>>(token: S,
                            orig_token: S,
                            start: usize,
                            end: usize) -> Self {
            SimpleToken {
                token: token.into(),
                original_token: orig_token.into(),
                start_pos: start,
                end_pos: end
            }

    }

    pub fn get_original_token(&self) -> &str {
        &self.original_token
    }

    pub fn get_lc_original_token(&self) -> String {
        self.original_token.to_lowercase()
    }

}


#[cfg(test)]
mod test {
    use std::assert_eq;

    use super::*;


    #[test]
    fn test_lower_case() {
        let tests = vec![
            ("Orange", "orange"),
            ("Apple", "apple"),
            ("pear", "pear")
        ];
        for test in tests {
            let st = SimpleToken::new(test.0, test.0, 1, 2);
            assert_eq!(test.1, st.get_lc_original_token());
        }

    }

}
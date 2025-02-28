//! A simple module that tests if a word is a stop word.
//! 
//! Stop words are the words that are filtered out (i.e. stopped) before or after processing of 
//! natural language data (text) because they are deemed insignificant.
//! 




pub fn is_stop(token: &str) -> bool {
    const STOP: &[&str] = &[
        "a", "the", "and","of", "in", "to", "on", "an", "with"
    ];
    STOP.contains(&token)
}


#[cfg(test)]
mod test {
    use std::assert_eq;

    use super::*;

    #[test]
    fn test_stop() {
        let tests = vec![
             ("a", true),
             ("the", true),
             ("and", true),
             ("of", true), 
             ("in", true),
             ("to", true),
             ("on", true),
             ("an", true),
             ("with", true),
             ("tada", false),
             ("red", false)
        ];
        for test in tests {
            let is_stop_w = is_stop(test.0);
            assert_eq!(test.1, is_stop_w);
        }
    }

}
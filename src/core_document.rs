use crate::simple_sentence::SimpleSentence;

pub struct CoreDocument {
    original_text: String,
    sentences: Vec<SimpleSentence>,
    //private final static Set<Character> sentenceEndPunctuation = Set.of('.', '!', '?');
}



impl CoreDocument {
    /// divide original text into sentences with boundaries on period, exclamation or question mark
    pub fn new(text: &str) -> Self {
        let mut ssentences = Vec::new();

        let mut start = 0;
        let chars: Vec<char> = text.chars().collect();
        for (i, &c) in chars.iter().enumerate() {
            if c == '.' || c == '!' || c == '?' {
                // Look ahead to include the period and any following space(s)
                let mut end = i + 1;
                while end < chars.len() && chars[end].is_whitespace() {
                    end += 1;
                }
                let sentence: String = chars[start..end].iter().collect();
                ssentences.push(SimpleSentence::new(sentence.trim_end(), start, end));
                start = end;
            }
        }
            // Add trailing sentence if any
        if start < chars.len() {
            let sentence: String = chars[start..].iter().collect();
            //result.push((start, chars.len(), sentence.trim_end()));
            ssentences.push(SimpleSentence::new(&sentence.trim_end(), start, chars.len()));
        }
    
        CoreDocument {
            original_text: text.into(),
            sentences: ssentences,
        }
    }

    pub fn get_sentences(&self) -> &[SimpleSentence] {
        &self.sentences
    }

    pub fn original_text(&self) -> &str {
        &self.original_text
    }
}

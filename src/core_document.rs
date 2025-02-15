use crate::simple_sentence::SimpleSentence;



pub struct CoreDocument {
    original_text: String,
    sentences: Vec<SimpleSentence>,
   
    //private final static Set<Character> sentenceEndPunctuation = Set.of('.', '!', '?');

}

impl CoreDocument {
    
    pub fn new(text: &str) -> Self {
        let mut ssentences = Vec::new();
        // Collect positions of '.', '!', and '?' - sentence boundaries
        let mut results = Vec::new();
        let mut start = 0;

        for (i, c) in text.char_indices() {
            if c == '.' || c == '!' || c == '?' {
                // Extract the substring (trim leading/trailing spaces)
                let part = text[start..=i].trim();

                if !part.is_empty() {
                    ssentences.push(SimpleSentence::new(part, start, i));
                }

                start = i + 1; // Move start to the next character
            }
        }

        // Handle any trailing text after the last punctuation
        if start < text.len() {
            let part = text[start..].trim();
            if !part.is_empty() {
                results.push((part, start));
                ssentences.push(SimpleSentence::new(part, start, text.len() -1));
            }
        }
        CoreDocument {
            original_text: text.into(),
            sentences: ssentences
        }
    }

    pub fn get_sentences(&self) -> &[SimpleSentence] {
        &self.sentences
    } 
}
//! negex
//! Implementation of the NegEx algorithm 
//! Heuristics for detecting negation in clinical texts,
//! Chapman WW, et al. A simple algorithm for identifying negated findings and diseases in discharge summaries. 
//! J Biomed Inform. 2001;34(5):301-10. PMID:12123149.
use std::collections::HashSet;

pub enum NegationType {
    Pre,
    Post,
    Terminator,
}

pub struct NegEx {
    pre_triggers: HashSet<String>,
    post_triggers: HashSet<String>,
    terminators: HashSet<String>,
}

impl NegEx {
    pub fn from_embedded() -> Self {
        // Embed the file at compile time
        let data = include_str!("../../data/negex_triggers.tsv");
        
        let mut pre = HashSet::new();
        let mut post = HashSet::new();
        let mut term = HashSet::new();

        for line in data.lines().filter(|l| !l.is_empty() && !l.starts_with('#')) {
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() >= 2 {
                let phrase = parts[0].to_lowercase();
                match parts[1] {
                    "PREN" => pre.insert(phrase),
                    "POST" => post.insert(phrase),
                    "TERM" => term.insert(phrase),
                    _ => false,
                };
            }
        }

        Self {
            pre_triggers: pre,
            post_triggers: post,
            terminators: term,
        }
    }

    /// Checks if a hit at a specific range is negated within a token slice.
    pub fn is_negated(&self, tokens: &[&str], hit_range: std::ops::Range<usize>) -> bool {
        // 1. Check Pre-negation (Look back 5 tokens)
        let start_lookback = hit_range.start.saturating_sub(5);
        for i in (start_lookback..hit_range.start).rev() {
            let word = tokens[i];
            if self.terminators.contains(word) { break; }
            if self.pre_triggers.contains(word) { return true; }
        }

        // 2. Check Post-negation (Look forward 5 tokens)
        let end_lookforward = std::cmp::min(tokens.len(), hit_range.end + 5);
        for i in hit_range.end..end_lookforward {
            let word = tokens[i];
            if self.terminators.contains(word) { break; }
            if self.post_triggers.contains(word) { return true; }
        }

        false
    }
}
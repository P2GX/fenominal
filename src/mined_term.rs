
use ontolius::base::TermId;

use crate::simple_token::SimpleToken;


/// DetailedMinedTerm in Java
#[derive(Clone)]
pub struct MinedTerm {
    tokens: Vec<SimpleToken>,
    term_id: TermId,
    start_pos: usize,
    end_pos: usize,
    matching_string: String,
    is_observed: bool

    /*
     private final List<SimpleToken> tokens;
    private final TermId tid;
    private final Map<Decoration, String> decorations;
    private final int startpos;
    private final int endpos;
    private final double similarity;
    private final String matchingString;
    /** If true, term was observed; if false, term was excluded */
    private final boolean isPresent;
 */
}

impl MinedTerm {

    pub fn new(tokens: Vec<SimpleToken>,
                tid: TermId,
                start: usize,
                end: usize,
                matching: &str,
                observed: bool) -> Self {
            MinedTerm {
                tokens: tokens,
                term_id: tid,
                start_pos: start,
                end_pos: end,
                matching_string: matching.into(),
                is_observed: observed
            }
    }


    pub fn get_start_pos(&self) -> usize {
        self.start_pos
    }

    pub fn get_end_pos(&self) -> usize {
        self.end_pos
    }
}


impl std::fmt::Display for MinedTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MinedTerm(id: {} [{}-{}])", self.term_id, self.start_pos, self.end_pos)
    }
}
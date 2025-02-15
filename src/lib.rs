

mod core_document;
mod fenominal_traits;
mod hpo;
mod mined_term;
mod simple_sentence;
mod simple_token;
mod stopwords;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

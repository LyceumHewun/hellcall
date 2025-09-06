use strsim::levenshtein;

pub struct FuzzyMatcher {
    dict: Vec<String>,
}

impl FuzzyMatcher {
    pub fn new(dict: Vec<String>) -> Self {
        Self { dict }
    }

    pub fn match_str(&self, input: &str) -> Option<&str> {
        self.dict
            .iter()
            .min_by_key(|x| levenshtein(x, input))
            .map(|x| x.as_str())
    }
}

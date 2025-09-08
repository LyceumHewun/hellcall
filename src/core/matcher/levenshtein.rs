use strsim::{jaro_winkler, levenshtein};

pub struct LevenshteinMatcher {
    dict: Vec<String>,
}

impl LevenshteinMatcher {
    pub fn new(dict: Vec<String>) -> Self {
        Self { dict }
    }

    pub fn match_str(&self, input: &str) -> Option<&str> {
        let input_norm = self.normalize(input);

        let max_levenshtein = 1;   // 编辑距离阈值
        let min_jaro = 0.80;         // jaro-winkler 最低相似度
        let alpha = 0.7;             // 权重：levenshtein
        let beta = 0.3;              // 权重：jaro_winkler

        self.dict
            .iter()
            .filter_map(|candidate| {
                let cand_norm = self.normalize(candidate);

                let lev = levenshtein(&cand_norm, &input_norm);
                let jw = jaro_winkler(&cand_norm, &input_norm);

                // 编辑距离过大 或 相似度过低 → 过滤掉
                if lev > max_levenshtein && jw < min_jaro {
                    return None;
                }

                // 综合分数（越小越好）
                let score = alpha * (lev as f64) + beta * (1.0 - jw);
                Some((candidate.as_str(), score))
            })
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .map(|(best, _)| best)
    }

    fn normalize(&self, input: &str) -> String {
        input
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>()
    }
}

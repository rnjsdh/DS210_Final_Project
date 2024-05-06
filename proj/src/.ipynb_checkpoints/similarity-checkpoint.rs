use std::collections::HashSet;

pub fn jaccard_similarity(set1: &HashSet<String>, set2: &HashSet<String>) -> f64 {
    let intersection: HashSet<_> = set1.intersection(set2).collect();
    let union: HashSet<_> = set1.union(set2).collect();
    if union.is_empty() {
        0.0
    } else {
        intersection.len() as f64 / union.len() as f64
    }
}

pub fn jaccard_dissimilarity(set1: &HashSet<String>, set2: &HashSet<String>) -> f64 {
    1.0 - jaccard_similarity(set1, set2)
}

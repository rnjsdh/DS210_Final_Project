mod similarity;
mod file_io;

use similarity::{jaccard_dissimilarity, jaccard_similarity};
use std::collections::HashMap;
use std::collections::HashSet;
use file_io::load_graph_from_csv;

#[derive(Debug)]
struct Graph {
    nodes: HashMap<String, HashSet<String>>,
}

impl Graph {
    fn new() -> Graph {
        Graph {
            nodes: HashMap::new(),
        }
    }

    fn add_movie(&mut self, title: &str, actors: Vec<String>) {
        self.nodes.insert(title.to_string(), actors.into_iter().collect());
    }

    fn get_neighbors(&self, title: &str) -> Option<&HashSet<String>> {
        self.nodes.get(title)
    }
    
    fn get_most_similar(&self) -> Option<(String, String, f64)> {
        let mut max_similarity = 0.0;
        let mut movie_pair = ("".to_string(), "".to_string());
        for title1 in self.nodes.keys() {
            for title2 in self.nodes.keys() {
                if title1 == title2 {
                    continue;
                }
                if let Some(similarity) = self.jaccard_similarity(title1, title2) {
                    if similarity > max_similarity {
                        max_similarity = similarity;
                        movie_pair = (title1.clone(), title2.clone());
                    }
                }
            }
        }
        if max_similarity > 0.0 {
            Some((movie_pair.0, movie_pair.1, max_similarity))
        } else {
            None
        }
    }

    fn get_most_dissimilar(&self) -> Option<(String, String, f64)> {
        let mut max_dissimilarity = 0.0;
        let mut movie_pair = ("".to_string(), "".to_string());
        for title1 in self.nodes.keys() {
            for title2 in self.nodes.keys() {
                if title1 == title2 {
                    continue;
                }
                if let Some(dissimilarity) = self.jaccard_dissimilarity(title1, title2) {
                    if dissimilarity > max_dissimilarity {
                        max_dissimilarity = dissimilarity;
                        movie_pair = (title1.clone(), title2.clone());
                    }
                }
            }
        }
        if max_dissimilarity > 0.0 {
            Some((movie_pair.0, movie_pair.1, max_dissimilarity))
        } else {
            None
        }
    }
    
    fn jaccard_similarity(&self, title1: &str, title2: &str) -> Option<f64> {
        let set1 = self.get_neighbors(title1)?;
        let set2 = self.get_neighbors(title2)?;
        Some(jaccard_similarity(set1, set2))
    }

    fn jaccard_dissimilarity(&self, title1: &str, title2: &str) -> Option<f64> {
        let set1 = self.get_neighbors(title1)?;
        let set2 = self.get_neighbors(title2)?;
        Some(jaccard_dissimilarity(set1, set2))
    }
}
    
fn main() {
    let graph = load_graph_from_csv("netflix_titles.csv");

    println!("Graph loaded with {} movies.", graph.nodes.len());
    
    if let Some((title1, title2, similarity)) = graph.get_most_similar() {
        println!(
            "The most similar movies are '{}' and '{}' with a similarity score of {}.",
            title1, title2, similarity
        );
    } else {
        println!("No similar movies found.");
    }

    if let Some((title1, title2, dissimilarity)) = graph.get_most_dissimilar() {
        println!(
            "The most dissimilar movies are '{}' and '{}' with a dissimilarity score of {}.",
            title1, title2, dissimilarity
        );
    } else {
        println!("No dissimilar movies found.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_most_similar() {
        let mut graph = Graph::new();
        graph.add_movie("Movie 1", vec!["Actor 1".to_string(), "Actor 2".to_string()]);
        graph.add_movie("Movie 2", vec!["Actor 2".to_string(), "Actor 3".to_string()]);
        graph.add_movie("Movie 3", vec!["Actor 4".to_string(), "Actor 5".to_string()]);

        if let Some((title1, title2, similarity)) = graph.get_most_similar() {
            assert_eq!(title1, "Movie 1");
            assert_eq!(title2, "Movie 2");
            assert_eq!(similarity, 1.0 / 3.0);
        } else {
            panic!("No similar movies found.");
        }
    }

    #[test]
    fn test_get_most_dissimilar() {
        let mut graph = Graph::new();
        graph.add_movie("Movie 1", vec!["Actor 1".to_string(), "Actor 2".to_string()]);
        graph.add_movie("Movie 2", vec!["Actor 2".to_string(), "Actor 3".to_string()]);
        graph.add_movie("Movie 3", vec!["Actor 4".to_string(), "Actor 5".to_string()]);

        if let Some((title1, title2, dissimilarity)) = graph.get_most_dissimilar() {
            assert_eq!(title1, "Movie 1");
            assert_eq!(title2, "Movie 3");
            assert_eq!(dissimilarity, 1.0);
        } else {
            panic!("No dissimilar movies found.");
        }
    }
}
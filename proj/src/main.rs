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

    
fn main() {
    let graph = load_graph_from_csv("netflix_titles.csv");

    println!("Graph loaded with {} movies.", graph.nodes.len());
}


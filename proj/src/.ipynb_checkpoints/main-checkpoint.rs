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

fn main() {
    let graph = load_graph_from_csv("netflix_titles.csv");

    println!("Graph loaded with {} movies.", graph.nodes.len());
}


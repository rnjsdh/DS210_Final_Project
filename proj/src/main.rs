mod similarity;
mod file_io;

use similarity::{jaccard_dissimilarity, jaccard_similarity};
use std::collections::HashMap;
use std::collections::HashSet;
use file_io::load_graph_from_csv;

fn main() {
    let graph = load_graph_from_csv("netflix_titles.csv");

    println!("Graph loaded with {} movies.", graph.nodes.len());
}


use std::fs::File;
use std::io::{self, BufRead};
use super::Graph;

pub fn load_graph_from_csv(path: &str) -> Graph {
    let mut graph = Graph::new();
    if let Ok(file) = File::open(path) {
        for line in io::BufReader::new(file).lines().skip(1) {
            if let Ok(record) = line {
                let parts: Vec<&str> = record.split(',').collect();
                if parts.len() > 2 {
                    let title = parts[2].trim();
                    let actors: Vec<String> = parts[1]
                        .split('|')
                        .map(|s| s.trim().to_string())
                        .collect();
                    graph.add_movie(title, actors);
                }
            }
        }
    }
    graph
}
    
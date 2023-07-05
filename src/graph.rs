use std::clone::Clone;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Debug)]
pub struct Node<T> {
    pub id: T,
    pub edges: Vec<T>,
    pub inverse_edges: Vec<T>,
}

impl<T> Node<T> {
    pub fn new(id: T, edges: Vec<T>, inverse_edges: Vec<T>) -> Self {
        Self {
            id,
            edges,
            inverse_edges,
        }
    }
}

#[derive(Debug)]
pub struct Graph<T: Clone + Eq + std::hash::Hash> {
    pub nodes: HashMap<T, Node<T>>,
}

impl<T> Graph<T>
where
    T: FromStr + Clone + Eq + std::hash::Hash,
    <T as FromStr>::Err: std::fmt::Debug,
{
    pub fn new() -> Self {
        Self {
            nodes: HashMap::default(),
        }
    }
    pub fn add_node(&mut self, id: T) {
        if !self.nodes.contains_key(&id) {
            self.nodes
                .insert(id.clone(), Node::new(id.clone(), Vec::new(), Vec::new()));
        }
    }
    pub fn add_edge(&mut self, from: T, to: T) {
        self.add_node(from.clone());
        self.add_node(to.clone());
        self.nodes.get_mut(&from).unwrap().edges.push(to.clone());
        self.nodes.get_mut(&to).unwrap().inverse_edges.push(from);
    }
    pub fn load_from_directed_edges_file(&mut self, path: &str) {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line.unwrap();
            let mut iter = line.split_whitespace();
            let from = iter
                .next()
                .unwrap()
                .parse::<T>()
                .expect("Failed to parse 'from' node");
            let to = iter
                .next()
                .unwrap()
                .parse::<T>()
                .expect("Failed to parse 'to' node");
            self.add_edge(from, to);
        }
    }
}

// Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph() {
        let mut graph: Graph<usize> = Graph::new();
        graph.load_from_directed_edges_file("data/SCC.txt");
        assert_eq!(graph.nodes.len(), 875714);
    }
}

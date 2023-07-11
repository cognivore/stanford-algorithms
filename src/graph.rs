use fxhash::FxHashMap as HashMap;
use fxhash::FxHashSet as HashSet;
use std::clone::Clone;
use std::collections::VecDeque;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct Node<T> {
    pub id: T,
    pub edges: Vec<T>,
    pub inverse_edges: Vec<T>,
    pub seen: bool,
    pub processed: bool,
}

impl<T> Node<T> {
    pub fn new(id: T, edges: Vec<T>, inverse_edges: Vec<T>) -> Self {
        Self {
            id,
            edges,
            inverse_edges,
            seen: false,
            processed: false,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Graph<T: Clone + Eq + Hash> {
    pub nodes: HashMap<T, Node<T>>,
}

impl<T> Graph<T>
where
    T: FromStr + Clone + Eq + Hash,
    <T as FromStr>::Err: std::fmt::Debug,
{
    pub fn new() -> Self {
        Self {
            nodes: HashMap::default(),
        }
    }
    pub fn from_nodes(nodes: HashMap<T, Node<T>>) -> Self {
        Self { nodes }
    }
    pub fn new_node(&mut self, id: T) {
        if !self.nodes.contains_key(&id) {
            self.nodes
                .insert(id.clone(), Node::new(id.clone(), Vec::new(), Vec::new()));
        }
    }
    pub fn add_node(&mut self, node: &T) {
        self.new_node(node.clone());
    }
    pub fn add_edge(&mut self, from: T, to: T) {
        self.new_node(from.clone());
        self.new_node(to.clone());
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

pub fn dfs_finish_ord<T, Y>(
    graph: &mut Graph<T>,
    trajectory: &VecDeque<T>,
    neighbours: fn(&Node<T>) -> Vec<T>,
    search: fn(&Node<T>) -> Option<Y>,
) -> (Option<Y>, HashMap<T, i64>)
where
    T: Clone + Eq + Hash,
{
    let (y, finish_times) = dfs_finish_time(graph, trajectory, neighbours, search);
    let mut finish_order = HashMap::default();
    for (node, time) in finish_times {
        finish_order.insert(node, graph.nodes.len() as i64 - time - 1);
    }
    (y, finish_order)
}

pub fn dfs_finish_time<T, Y>(
    graph: &mut Graph<T>,
    trajectory: &VecDeque<T>,
    neighbours: fn(&Node<T>) -> Vec<T>,
    search: fn(&Node<T>) -> Option<Y>,
) -> (Option<Y>, HashMap<T, i64>)
where
    T: Clone + Eq + Hash,
{
    let mut finish_times = HashMap::default();
    let mut time = 0;
    let mut seen = HashSet::default();
    let mut stack = VecDeque::new();
    let mut trajectory = trajectory.clone();
    let mut result = None;
    stack.push_back(trajectory.front().unwrap().clone());
    loop {
        if stack.is_empty() {
            loop {
                if trajectory.is_empty() {
                    return (result, finish_times);
                }
                let node = trajectory.pop_front().unwrap();
                if !seen.contains(&node) {
                    stack.push_back(node);
                    break;
                }
            }
        } else {
            let node = stack.pop_back().unwrap();
            if !seen.contains(&node) {
                seen.insert(node.clone());
                stack.push_back(node.clone());
                let node = graph.nodes.get(&node).unwrap();
                let neighbours = neighbours(node);
                for neighbour in neighbours {
                    if !seen.contains(&neighbour) {
                        stack.push_back(neighbour);
                    }
                }
            } else {
                if !finish_times.contains_key(&node) {
                    finish_times.insert(node.clone(), time);
                    time += 1;
                }
                if let Some(y) = search(graph.nodes.get(&node).unwrap()) {
                    result = Some(y);
                }
            }
        }
    }
}

pub fn irrel<T>(_node: &Node<T>) -> Option<bool> {
    None
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

    #[test]
    fn test_toposort_dfs() {
        // A simple graph with 4 vertices
        let mut graph: Graph<usize> = Graph::new();
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(3, 4);
        graph.add_edge(2, 4);
        // Get all possible trajectories as all possible transpositions of vec![1,2,3,4]
        let trajectories: Vec<VecDeque<usize>> = vec![
            vec![1, 2, 3, 4],
            vec![1, 3, 2, 4],
            vec![4, 1, 2, 3],
            vec![2, 3, 1, 4],
        ]
        .into_iter()
        .map(|v| v.into_iter().collect())
        .collect();

        //let trajectory: VecDeque<usize> = graph.nodes.keys().cloned().collect();
        for trajectory in trajectories {
            let (_, processed) =
                dfs_finish_ord(&mut graph, &trajectory, |node| node.edges.clone(), irrel);
            // dbg!(&processed);
            assert!(processed.get(&1).unwrap() == &0);
            assert!(processed.get(&2).unwrap() == &1 || processed.get(&3).unwrap() == &1);
            assert!(processed.get(&2).unwrap() == &2 || processed.get(&3).unwrap() == &2);
            assert!(processed.get(&4).unwrap() == &3);
        }
    }

    #[test]
    fn test_toposort_dfs_terminates() {
        // Cyclic graph with 3 nodes
        let mut graph: Graph<usize> = Graph::new();
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 1);
        let trajectory: VecDeque<usize> = graph.nodes.keys().cloned().collect();
        let (result, _) = dfs_finish_ord(&mut graph, &trajectory, |node| node.edges.clone(), irrel);
        assert!(result.is_none());
    }
}

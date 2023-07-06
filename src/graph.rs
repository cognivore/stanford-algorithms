use fxhash::FxHashMap as HashMap;
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

pub fn dfs_topo<T, Y>(
    graph: &mut Graph<T>,
    trajectory: &VecDeque<T>,
    neighbours: fn(&Node<T>) -> Vec<T>,
    search: fn(&Node<T>) -> Option<Y>,
) -> (Option<Y>, HashMap<usize, T>)
where
    T: Clone + Eq + Hash,
{
    let mut queue: VecDeque<T> = trajectory.clone();
    assert_eq!(queue.len(), graph.nodes.len());
    let mut stack: Vec<T> = vec![];
    //
    // let mut processed: Vec<T> = Vec::new();
    // let mut seen: Vec<T> = Vec::new();
    let mut top_sort: HashMap<usize, T> = HashMap::default();
    let mut i = trajectory.len();
    //
    let is_start: bool = true;
    // Diagnostics
    let mut max_delta: u128 = 0;
    let mut time_previous: std::time::Instant = std::time::Instant::now();
    let mut diag_i: usize = 0;
    while !queue.is_empty() {
        let node_id = if stack.is_empty() {
            queue.pop_front().unwrap()
        } else {
            stack.pop().unwrap()
        };
        let node: &mut Node<T> = graph.nodes.get_mut(&node_id).unwrap();
        // Check that HashMap's values contain node_id
        if !node.processed {
            // Generate pseudorandum number between 0 and 1000
            diag_i = diag_i + 1;
            if diag_i % 1000 == 0 {
                eprint!(".");
                let time_now: std::time::Instant = std::time::Instant::now();
                if time_now.duration_since(time_previous).as_millis() > max_delta {
                    eprintln!("[dfs_topo] max delta has increased to: {:?}", max_delta);
                    max_delta = time_now.duration_since(time_previous).as_millis();
                }
                time_previous = time_now;
            }

            node.seen = true;
            if let Some(result) = search(node) {
                return (Some(result), top_sort);
            }
            if is_start {
                stack.push(node_id.clone());
            }
            let current_neighbours: Vec<T> = neighbours(node)
                .iter()
                .filter(|neighbour| !graph.nodes.get(&neighbour).unwrap().processed)
                .cloned()
                .collect();
            for neighbour in &current_neighbours {
                if !graph.nodes.get(&neighbour).unwrap().seen {
                    stack.push(neighbour.clone());
                }
            }
            let unseen_neighbours: Vec<T> = current_neighbours
                .iter()
                .filter(|neighbour| !graph.nodes.get(&neighbour).unwrap().seen)
                .cloned()
                .collect();
            if unseen_neighbours.is_empty() {
                i = i - 1;
                let node = graph.nodes.get_mut(&node_id).unwrap();
                node.processed = true;
                top_sort.insert(i, node_id);
            }
        }
    }
    // Unset seen and processed flags
    for node in graph.nodes.values_mut() {
        node.seen = false;
        node.processed = false;
    }
    (None, top_sort)
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
                dfs_topo(&mut graph, &trajectory, |node| node.edges.clone(), irrel);
            assert!(processed.get(&0).unwrap() == &1);
            assert!(processed.get(&1).unwrap() == &2 || processed.get(&1).unwrap() == &3);
            assert!(processed.get(&2).unwrap() == &2 || processed.get(&2).unwrap() == &3);
            assert!(processed.get(&3).unwrap() == &4);
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
        let (result, _) = dfs_topo(&mut graph, &trajectory, |node| node.edges.clone(), irrel);
        assert!(result.is_none());
    }
}

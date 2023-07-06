crate::entry_point!("kosajaru", main);
use crate::graph::{dfs_topo, irrel, Graph, Node};
use fxhash::FxHashMap as HashMap;
use std::collections::VecDeque;
use std::hash::Hash;
use std::str::FromStr;

pub fn main() {
    let mut graph: Graph<usize> = Graph::new();
    graph.load_from_directed_edges_file("data/SCC.txt");

    let sccs = kosaraju(&mut graph);
    // Get top 5 sized SCCs
    let mut outcome_sorted = sccs.iter().map(|x| x.nodes.len()).collect::<Vec<usize>>();
    outcome_sorted.sort();
    let outcome = outcome_sorted
        .into_iter()
        .rev()
        .take(5)
        .collect::<Vec<usize>>();
    // Print their sizes
    println!("{:?}", outcome);
}

pub fn kosaraju(graph: &mut Graph<usize>) -> Vec<Graph<usize>> {
    let trajectory1: VecDeque<usize> = graph.nodes.keys().cloned().collect();
    let (_, topo_sort1) = dfs_topo(graph, &trajectory1, |x| x.inverse_edges.clone(), irrel);
    // Trajectory 2 is topo_sort1 from lowest vertex to highest
    // dbg!(&topo_sort1);
    let mut trajectory2 = VecDeque::new();
    for i in 0..graph.nodes.len() {
        // dbg!(&trajectory2);
        trajectory2.push_back(topo_sort1.get(&i).unwrap().clone());
        // dbg!(&trajectory2);
        // println!("************************");
    }
    // dbg!(&graph);
    // dbg!(&trajectory2);
    let sccs = dfs_scc(graph, &trajectory2, |x| x.edges.clone());
    sccs
}

pub fn dfs_scc<T>(
    graph: &mut Graph<T>,
    trajectory: &VecDeque<T>,
    neighbours: fn(&Node<T>) -> Vec<T>,
) -> Vec<Graph<T>>
where
    T: Clone + Eq + Hash + FromStr + std::fmt::Debug,
    <T as FromStr>::Err: std::fmt::Debug,
{
    let mut queue: VecDeque<T> = trajectory.clone();
    assert_eq!(queue.len(), graph.nodes.len());
    let mut stack: Vec<T> = vec![];
    //
    // let mut top_sort: HashMap<usize, T> = HashMap::default();
    let mut i = trajectory.len();
    //
    let mut current_scc: HashMap<T, Node<T>> = HashMap::default();
    let mut ys = vec![];
    let mut is_start: bool = true;
    // Diagnostics
    let mut max_delta: u128 = 0;
    let mut time_previous: std::time::Instant = std::time::Instant::now();
    let mut diag_i: usize = 0;
    // dbg!(&queue);
    while !queue.is_empty() {
        let node_id = if stack.is_empty() {
            // dbg!("stack is EMPTY, popping QUEUE");
            let mut node_id_candidate = queue.pop_front().unwrap();
            let mut node_candidate = graph.nodes.get(&node_id_candidate).unwrap();
            while !queue.is_empty() && node_candidate.processed {
                node_id_candidate = queue.pop_front().unwrap();
                node_candidate = graph.nodes.get(&node_id_candidate).unwrap();
            }
            // dbg!(is_start);
            if !is_start {
                // dbg!("ADDING STUFF");
                // dbg!(&current_scc);
                ys.push(Graph::from_nodes(current_scc));
                // dbg!(&ys);
                current_scc = HashMap::default();
            }
            is_start = true;
            // dbg!("RETURNING CANDIDATE");
            node_id_candidate
        } else {
            // dbg!("stack is not empty, popping");
            let next = stack.pop().unwrap();
            // dbg!("next: {:?}", &next);
            next
        };
        // Check that HashMap's values contain node_id
        let node = graph.nodes.get_mut(&node_id).unwrap();
        if !node.processed {
            node.seen = true;
            // Generate pseudorandum number between 0 and 1000
            diag_i = diag_i + 1;
            if diag_i % 1000 == 0 {
                let glitch = graph.nodes.get(&node_id).unwrap();
                dbg!(&glitch);
                for neighbour in neighbours(glitch) {
                    dbg!(graph.nodes.get(&neighbour).unwrap());
                }
                eprintln!("*************************************************");
                let time_now: std::time::Instant = std::time::Instant::now();
                if time_now.duration_since(time_previous).as_millis() > max_delta {
                    eprintln!("[dfs_topo] max delta has increased to: {:?}", max_delta);
                    max_delta = time_now.duration_since(time_previous).as_millis();
                }
                time_previous = time_now;
            }

            let node = graph.nodes.get(&node_id).unwrap();
            if is_start {
                stack.push(node_id.clone());
            }
            is_start = false;
            let current_neighbours: Vec<T> = neighbours(node)
                .iter()
                .filter(|neighbour| !graph.nodes.get(&neighbour).unwrap().processed)
                .cloned()
                .collect();
            for neighbour in &current_neighbours {
                if !graph.nodes.get(&neighbour).unwrap().processed {
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
                current_scc.insert(
                    node_id.clone(),
                    Node::new(
                        node_id.clone(),
                        node.edges.clone(),
                        node.inverse_edges.clone(),
                    ),
                );
                // top_sort.insert(i, node_id);
            }
            // dbg!("ending stack: {:?}", &stack);
            // dbg!("ending queue: {:?}", &queue);
            //dbg!("processed: {:?}", &processed);
            //dbg!("seen: {:?}", &seen);
        }
    }
    if !current_scc.is_empty() {
        ys.push(Graph::from_nodes(current_scc));
    }
    ys
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orion_is_scc_2() {
        // Graph that looks like Orion constellation
        /*
         *
         *     1 -- 2
         *      \    \
         *       \    \
         *        3 -- 4
         *              `-- 5
         *
         */
        let mut graph: Graph<usize> = Graph::new();
        graph.add_edge(2, 1);
        graph.add_edge(1, 3);
        graph.add_edge(3, 4);
        graph.add_edge(4, 2);
        graph.add_edge(5, 4);
        let mut loopy: Graph<usize> = Graph::new();
        loopy.add_edge(2, 1);
        loopy.add_edge(1, 3);
        loopy.add_edge(3, 4);
        loopy.add_edge(4, 2);
        let mut taily: Graph<usize> = Graph::new();
        taily.add_edge(5, 4);
        //
        let sccs = kosaraju(&mut graph);
        let mut outcome_sorted = sccs.iter().map(|x| x.nodes.len()).collect::<Vec<usize>>();
        outcome_sorted.sort();
        let outcome = outcome_sorted.into_iter().rev().collect::<Vec<usize>>();
        assert_eq!(outcome, vec![4, 1]) // ;
    }

    #[test]
    fn test_8_16() {
        /*
         *
         * 1 ---> 3 ---> 11--->6
         *  ^    /         \   ^\
         *   \  /           v / |
         *    \v            >8  |
         *    5            / ^  |
         *    /\    ,-----`  |  |
         *   /  \  /         |  |
         *  v    v/          |  |
         *  7--->9           |  |
         *  ^   /\           \  |
         *  \  /  \           \ |
         *   \v    v           \v
         *    4<---2----------->10
         *
         */
        let mut graph: Graph<usize> = Graph::new();
        graph.add_edge(1, 3);
        graph.add_edge(2, 4);
        graph.add_edge(2, 10);
        graph.add_edge(3, 5);
        graph.add_edge(3, 11);
        graph.add_edge(4, 7);
        graph.add_edge(5, 1);
        graph.add_edge(5, 7);
        graph.add_edge(5, 9);
        graph.add_edge(6, 10);
        graph.add_edge(7, 9);
        graph.add_edge(8, 6);
        graph.add_edge(9, 2);
        graph.add_edge(9, 4);
        graph.add_edge(9, 8);
        graph.add_edge(10, 8);
        graph.add_edge(11, 6);
        graph.add_edge(11, 8);
        let sccs = kosaraju(&mut graph);
        assert_eq!(sccs.len(), 4);
        let mut outcome_sorted = sccs.iter().map(|x| x.nodes.len()).collect::<Vec<usize>>();
        outcome_sorted.sort();
        assert_eq!(outcome_sorted, vec![1, 3, 3, 4]);
    }

    #[test]
    fn test_you_got_me_well_on_a_bidirectional_node() {
        /*
         *
         *
         *
         *   1 <--- 2
         *     --->
         *
         *
         *
         */
        let mut graph: Graph<usize> = Graph::new();
        graph.add_edge(1, 2);
        graph.add_edge(2, 1);
        let sccs = kosaraju(&mut graph);
        assert_eq!(sccs.len(), 1);
        assert_eq!(sccs[0].nodes.len(), 2);
    }

    #[test]
    fn test_complex_bidirectional() {
        /*
         *
         *   ,-------------,
         *   v             |
         *   1 <--- 2 ---> 3 ---> 4
         *   |             ^ <---'
         *   `-------------`
         *
         *
         */
        let mut graph: Graph<usize> = Graph::new();
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 1);
        graph.add_edge(1, 3);
        graph.add_edge(3, 4);
        graph.add_edge(4, 3);
        let sccs = kosaraju(&mut graph);
        assert_eq!(sccs.len(), 1);
    }
}

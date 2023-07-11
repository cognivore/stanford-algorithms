crate::entry_point!("kosajaru", main);
use crate::graph::{dfs_finish_ord, irrel, Graph, Node};
use fxhash::FxHashMap as HashMap;
use fxhash::FxHashSet as HashSet;
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
    let (_, topo_sort1) = dfs_finish_ord(graph, &trajectory1, |x| x.inverse_edges.clone(), irrel);
    // Trajectory 2 is topo_sort1 from lowest vertex to highest
    // dbg!(&topo_sort1);
    let sort_topo1 = topo_sort1
        .iter()
        .map(|(k, v)| (v.clone(), k))
        .collect::<HashMap<i64, &usize>>();
    let mut trajectory2 = VecDeque::new();
    for i in 0..sort_topo1.len() {
        let nid: usize = *sort_topo1.get(&(i as i64)).unwrap().clone();
        trajectory2.push_back(nid);
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
    let mut finish_times = HashMap::default();
    let mut time = 0;
    let mut stack = VecDeque::new();
    let mut trajectory = trajectory.clone();
    stack.push_back(trajectory.front().unwrap().clone());
    let mut current_scc = Graph::new();
    let mut sccs = Vec::new();
    let mut seen = HashSet::default();
    loop {
        // dbg!(&stack);
        // dbg!(&finish_times);
        if stack.is_empty() {
            loop {
                if trajectory.is_empty() {
                    sccs.push(current_scc);
                    return sccs;
                }
                let node = trajectory.pop_front().unwrap();
                if !seen.contains(&node) {
                    // dbg!(&trajectory);
                    sccs.push(current_scc);
                    current_scc = Graph::new();
                    stack.push_back(node);
                    break;
                }
            }
        }
        let node = stack.pop_back().unwrap();
        if !seen.contains(&node) {
            seen.insert(node.clone());
            // eprintln!("Pushing node {:?}", node);
            stack.push_back(node.clone());
            let node = graph.nodes.get(&node).unwrap();
            let neighbours = neighbours(node);
            for neighbour in neighbours {
                let n = neighbour.clone();
                if !(&neighbour == &node.id) && !seen.contains(&neighbour) {
                    stack.push_back(n);
                }
            }
        } else {
            if !finish_times.contains_key(&node) {
                finish_times.insert(node.clone(), time);
                time += 1;
                current_scc.add_node(&node);
            }
        }
    }
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
        // dbg!(&sccs);
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
         *   v             |      ,---,
         *   1 <--- 2 ---> 3 ---> 4<--`
         *   |             ^ <---'^
         *   `-------------`       \
         *                          \
         *                          `--> 5
         */
        let mut graph: Graph<usize> = Graph::new();
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 1);
        graph.add_edge(1, 3);
        graph.add_edge(3, 4);
        graph.add_edge(4, 3);
        graph.add_edge(4, 4);
        graph.add_edge(4, 5);
        graph.add_edge(5, 4);
        let sccs = kosaraju(&mut graph);
        assert_eq!(sccs.len(), 1);
    }
}

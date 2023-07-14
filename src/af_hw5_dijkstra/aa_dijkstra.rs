#![allow(dead_code)]

crate::entry_point!("dijkstra", main);
use petgraph::{algo::dijkstra, prelude::UnGraph, visit::GraphBase, Graph};
// fxhash is my HashMap now
use fxhash::FxHashMap as HashMap;

// Returns empty directed graph with node and edge weights of type i32
pub fn sample_graph() -> Graph<i32, i32> {
    Graph::new()
}

pub fn sample_undirected() -> UnGraph<i32, i32> {
    UnGraph::new_undirected()
}

type G = Graph<i32, i32>;

// Read a directed graph from a file which contains a list of nodes and edges in a form (node,
// weight). All the nodes have weight 1.
// Sample of the file:
// 1	80,982	163,8164	170,2620	145,648	200,8021	173,2069	92,647	26,4122	140,546	11,1913	160,6461	27,7905	40,9047	150,2183	61,9146	159,7420	198,1724	114,508	104,6647	30,4612	99,2367	138,7896	169,8700	49,2437	125,2909	117,2597	55,6399
// 2	42,1689	127,9365	5,8026	170,9342	131,7005	172,1438	34,315	30,2455	26,2328	6,8847	11,1873	17,5409	157,8643	159,1397	142,7731	182,7908	93,8177
// 3	57,1239	101,3381	43,7313	41,7212	91,2483	31,3031	167,3877	106,6521	76,7729	122,9640	144,285	44,2165	6,9006	177,7097	119,7711
// We parse it into a petgraph Graph to learn how to use petgraph.
pub fn read_graph_from_file(filename: &str) -> Graph<i32, i32> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    // First read the lines into an ordered adjacency vector, we'll be using iterator over it to
    // create the graph with add_node method, preserving the order of nodes.

    // Read line by line into a hashmap where key is the node index, which is the leftmost value,
    // and the value is a vector of tuples (node_index, weight).
    let mut adjacency_list: HashMap<usize, Vec<(usize, i32)>> = HashMap::default();
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let mut iter = line.split_whitespace();
        let node_index = iter.next().unwrap().parse::<usize>().unwrap();
        let mut edges: Vec<(usize, i32)> = Vec::new();
        for edge in iter {
            let mut edge_iter = edge.split(",");
            let target_index = edge_iter.next().unwrap().parse::<usize>().unwrap();
            let weight = edge_iter.next().unwrap().parse::<i32>().unwrap();
            edges.push((target_index - 1, weight));
        }
        adjacency_list.insert(node_index - 1, edges);
    }

    // Reenumerate the adjacency list to make sure that the node indices are in the range 0..n-1
    // It means that we also need to update the values of the hashmap if we change the keys.

    // First get sorted keys of the adjacency_list
    let mut keys: Vec<usize> = adjacency_list.keys().copied().collect();

    // Check if the keys are sorted and span the range 0..n-1. If they do, we're done with this
    // part of the algorithm.
    let mut changes = HashMap::default();
    keys.sort();
    for i in 0..keys.len() {
        if keys[i] != i {
            // We need to change the key and the value
            let old_key = keys[i];
            let new_key = i;
            let value = adjacency_list.remove(&old_key).unwrap();
            adjacency_list.insert(new_key, value);
            changes.insert(old_key, new_key);
        }
    }

    // Now we need to update the values of the hashmap
    for (_key, value) in adjacency_list.iter_mut() {
        for (target, _weight) in value.iter_mut() {
            if changes.contains_key(target) {
                *target = *changes.get(target).unwrap();
            }
        }
    }

    // Now we can create the graph
    let mut g = Graph::new();

    // Add nodes
    for i in 0..adjacency_list.len() {
        g.add_node(i as i32); // weight = node id just for easy debug
    }

    //Add edges
    for (node_index, edges) in adjacency_list.iter() {
        for (target_index, weight) in edges.iter() {
            g.add_edge(
                g.node_indices().nth(*node_index).unwrap(),
                g.node_indices().nth(*target_index).unwrap(),
                *weight,
            );
        }
    }

    g
}

fn get_distance(
    s: usize,
    t: usize,
    graph: &G,
    distances: &HashMap<<G as GraphBase>::NodeId, HashMap<<G as GraphBase>::NodeId, i32>>,
) -> i32 {
    let from = graph.node_indices().nth(s).unwrap();
    let to = graph.node_indices().nth(t).unwrap();
    let distances = distances.get(&from).unwrap();
    *distances.get(&to).unwrap()
}

pub fn main() {
    // Read dijkstraData.txt into graph in memory
    let graph = read_graph_from_file("data/dijkstraData.txt");
    // Run Dijkstra's algorithm on the graph
    let mut distances: HashMap<<G as GraphBase>::NodeId, HashMap<<G as GraphBase>::NodeId, i32>> =
        HashMap::default();
    for i in 0..graph.node_count() {
        let node_index = graph.node_indices().nth(i).unwrap();
        // let mut inner_distances: HashMap<<G as GraphBase>::NodeId, i32> = HashMap::default();
        /*
        for j in 0..graph.node_count() {
            let target_index = graph.node_indices().nth(j).unwrap();
            let distance = dijkstra(&graph, node_index, Some(target_index), |e| {
                *e.weight() as i32
            });
            // dbg!(&distance);
            // dbg!((&i, &j, &node_index, &target_index, graph.node_count()));
            if let Some(distance) = distance.get(&target_index) {
                inner_distances.insert(target_index, *distance);
            }
        }
        */
        // Get 7th node_index
        let target_index = graph.node_indices().nth(6).unwrap();
        let inner_distances = dijkstra(&graph, node_index, Some(target_index), |e| {
            *e.weight() as i32
        })
        .iter()
        .fold(HashMap::default(), |mut acc, (k, v)| {
            acc.insert(*k, *v);
            acc
        });
        distances.insert(node_index, inner_distances);
    }
    dbg!(&distances
        .get(&graph.node_indices().nth(0).unwrap())
        .unwrap());
    // dbg!(&distances);
    println!("1-7");
    println!("{}", get_distance(0, 6, &graph, &distances));
    println!("1-37");
    println!("{}", get_distance(0, 36, &graph, &distances));
    println!("1-59");
    println!("{}", get_distance(0, 58, &graph, &distances));
    println!("1-82");
    println!("{}", get_distance(0, 81, &graph, &distances));
    println!("1-99");
    println!("{}", get_distance(0, 98, &graph, &distances));
    println!("1-115");
    println!("{}", get_distance(0, 114, &graph, &distances));
    println!("1-133");
    println!("{}", get_distance(0, 132, &graph, &distances));
    println!("1-165");
    println!("{}", get_distance(0, 164, &graph, &distances));
    println!("1-188");
    println!("{}", get_distance(0, 187, &graph, &distances));
    println!("1-197");
    println!("{}", get_distance(0, 196, &graph, &distances));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_simple_well_connected_5() {
        let mut g = Graph::new();
        let n1 = g.add_node(0);
        let n2 = g.add_node(0);
        let n3 = g.add_node(0);
        let n4 = g.add_node(0);
        let n5 = g.add_node(0);
        let nodes = vec![n1, n2, n3, n4, n5];
        for i in &nodes {
            for j in &nodes {
                if i != j {
                    g.add_edge(*i, *j, i.index() + j.index());
                }
            }
        }
        let dj1 = dijkstra(&g, n1, None, |e| *e.weight());
        let dj15 = dijkstra(&g, n1, Some(n5), |e| *e.weight());
        for i in &nodes {
            assert_eq!(dj1.get(i).unwrap(), &i.index());
        }
        assert_eq!(dj15.get(&n5).unwrap(), &4);
    }

    #[test]
    fn test_hourglasses() {
        let mut g = Graph::new();
        for _ in 0..5 {
            g.add_node(0);
        }
        g.add_edge(
            g.node_indices().nth(0).unwrap(),
            g.node_indices().nth(1).unwrap(),
            1,
        );
        g.add_edge(
            g.node_indices().nth(1).unwrap(),
            g.node_indices().nth(2).unwrap(),
            5,
        );
        g.add_edge(
            g.node_indices().nth(2).unwrap(),
            g.node_indices().nth(0).unwrap(),
            7,
        );
        g.add_edge(
            g.node_indices().nth(3).unwrap(),
            g.node_indices().nth(2).unwrap(),
            3,
        );
        g.add_edge(
            g.node_indices().nth(3).unwrap(),
            g.node_indices().nth(4).unwrap(),
            2,
        );
        g.add_edge(
            g.node_indices().nth(4).unwrap(),
            g.node_indices().nth(2).unwrap(),
            1,
        );
        // This graph looks like this:
        // 0 -> 1
        // ^    |
        //  \   v
        //   ---2<--
        //      ^   \
        //      |    \
        //      4<----3
        let dj1 = dijkstra(&g, g.node_indices().nth(0).unwrap(), None, |e| *e.weight());
        assert_eq!(dj1.keys().len(), 3);
        let dj3 = dijkstra(&g, g.node_indices().nth(3).unwrap(), None, |e| *e.weight());
        assert_eq!(dj3.keys().len(), 5);
    }
}

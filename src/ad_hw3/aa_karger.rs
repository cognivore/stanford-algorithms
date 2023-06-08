crate::entry_point!("karger", main);

trait MinCuttable {
    fn min_cut(&mut self, rng: &mut impl rand::Rng) -> (usize, Vec<usize>, Vec<usize>);
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Node {
    id: usize,
    edges: Vec<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Graph {
    nodes: Vec<Node>,
}

impl Graph {
    fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    // Sample format for load_from_file:
    // 1	37	79	164	155	32	87	39	113	15	18	78	175	140	200	4	160	97	191	100	91	20	69	198	196
    // 2	123	134	10	141	13	12	43	47	3	177	101	179	77	182	117	116	36	103	51	154	162	128	30
    // 3	48	123	134	109	41	17	159	49	136	16	130	141	29	176	2	190	66	153	157	70	114	65	173	104	194	54
    fn load_from_file(&mut self, path: &str) {
        use std::fs::File;
        use std::io::BufRead;
        let file = File::open(path).unwrap();
        let reader = std::io::BufReader::new(file);
        for (id, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            let mut edges = Vec::new();
            for edge in line.split_whitespace().skip(1) {
                // !!! We go from 1-indexed to 0-indexed here !!!
                edges.push(edge.parse::<usize>().unwrap() - 1);
            }
            self.nodes.push(Node { id, edges });
        }
    }
}

impl MinCuttable for Graph {
    fn min_cut(&mut self, rng: &mut impl rand::Rng) -> (usize, Vec<usize>, Vec<usize>) {
        let graph = self;
        // Recursion? I barely even knew him!
        let mut removed = Vec::new();
        let mut last_known = 0;
        while graph.nodes.len() - removed.len() > 2 {
            // Sorry for this garbage, but we need a hacky way to delete nodes easily.
            // We delete them by clearing their edges.
            // That means that we can get empty nodes, which we need to ignore.
            let mut node_id = rng.gen_range(0..graph.nodes.len());
            while graph.nodes[node_id].edges.len() == 0 {
                // Random node from graph:
                node_id = rng.gen_range(0..graph.nodes.len());
            }
            // Random edge from node:
            let rand_edge_id = rng.gen_range(0..graph.nodes[node_id].edges.len());
            // dbg!("WE CHOSE", node_id, "WITH EDGE", rand_edge_id);
            let node2_id: usize = graph.nodes[node_id].edges[rand_edge_id].to_owned();
            // Thus, node2 is the node on the other side of the edge:
            let node2 = graph.nodes[node2_id.clone()].clone();
            // dbg!("Merging", &node2, "into", &graph.nodes[node_id]);
            // Merge node2 into node:
            for edge_i in node2.edges.iter() {
                // First add all edges from node2 to node.
                if *edge_i != node_id {
                    // dbg!("Adding edge", edge_i, "to node", node_id);
                    graph.nodes[node_id].edges.push(*edge_i);
                // Ignore self-loops.
                } else {
                    // dbg!("Ignoring self-loop", edge_i, "on node", node_id);
                }
                // Now replace all edges from X to node2 with edges to node.
                /* for node in graph.nodes[*edge_i].edges.iter_mut() {
                    dbg!("Checking edge", &node, "on node", edge_i);
                    if *node == node2.id {
                        dbg!("Replacing edge", &node, "with", &node_id);
                        *node = node_id;
                    }
                } */
                // Now replace all the node2 edges with node edges.
                /* for node in graph.nodes.iter_mut() {
                    for edge in node.edges.iter_mut() {
                        if *edge == node2_id {
                            *edge = node_id;
                        }
                    }
                } */
            }
            graph.nodes[node_id]
                .edges
                .retain(|&x| x != node2_id && x != node_id);

            for node_j in graph.nodes[node_id].edges.clone() {
                for edge_j in graph.nodes[node_j].edges.iter_mut() {
                    if *edge_j == node2_id {
                        *edge_j = node_id;
                    }
                }
            }

            // Delete all the mentions of node2 from node, because they are self-loops now.
            // Delete all the mentions of node from node too. Duh.

            // Delete all the edges from node2.
            graph.nodes[node2_id].edges = Vec::new();

            removed.push(node2.id);

            /*
            dbg!(
                "GRAPH NOW",
                &graph,
                "WE HAVE REMOVED",
                removed.len(),
                "NODES"
            );
            */

            last_known = node_id;
        }
        let mut retained = Vec::new();
        // Filter non-removed nodes:
        for node in &graph.nodes {
            if !removed.contains(&node.id) {
                retained.push(node.id);
            }
        }
        //dbg!("SUPPOSEDLY, THE LAST KNOWN IS", last_known);
        /*
        dbg!(format!(
            "[karger] Found min cut of {} with {} removed and {} retained",
            graph.nodes[last_known].edges.len(),
            removed.len(),
            retained.len()
        ));
        */
        (graph.nodes[last_known].edges.len(), removed, retained)
    }
}

fn main() {
    let mut graph = Graph::new();
    graph.load_from_file("data/kargerMinCut.txt");
    let n = graph.nodes.len();
    // To be (1 - (1 / n)) certain that we find the min cut, we need to run the algorithm n^2 * ln(n) times.
    let ln_ceil = (n as f64).ln().ceil() as usize;
    let iterations_to_certainty = (n * n * ln_ceil) as usize;
    let mut rng = rand::thread_rng();
    let mut min_cut = usize::MAX;
    for i in 0..iterations_to_certainty {
        let (cut, _, _) = graph.clone().min_cut(&mut rng);
        /* dbg!(
            "Found",
            cut,
            "at iteration",
            i,
            "out of",
            iterations_to_certainty
        ); */
        if cut == 0 {
            panic!("Impossible happened!");
        }
        if cut < min_cut {
            min_cut = cut;
        }
        if cut == 1 {
            break;
        }
        if i % 1_000 == 0 {
            println!(
                "Iteration {} out of {} ({}%) :: min cut: {}",
                i,
                iterations_to_certainty,
                (i as f64 / iterations_to_certainty as f64) * 100.0,
                min_cut
            );
        }
    }
    println!("min cut: {}", min_cut);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn triangle() {
        let mut graph = Graph::new();
        graph.nodes.push(Node {
            id: 0,
            edges: vec![1, 2],
        });
        graph.nodes.push(Node {
            id: 1,
            edges: vec![0, 2],
        });
        graph.nodes.push(Node {
            id: 2,
            edges: vec![0, 1],
        });
        let mut rng = rand::thread_rng();
        let (min, a, b) = graph.clone().min_cut(&mut rng);
        assert_eq!(min, 2);
        assert!((a.len() == 1 && b.len() == 2) || (a.len() == 2 && b.len() == 1));
        let mut triangle_from_file = Graph::new();
        triangle_from_file.load_from_file("data/kargerTriangle.txt");
        // triangle_from_file should be the same as graph:
        assert_eq!(triangle_from_file, graph.clone());
    }

    #[test]
    fn two_envelopes() {
        let mut graph = Graph::new();
        graph.load_from_file("data/kargerEnvelopes.txt");
        let mut rng = rand::thread_rng();
        let mut min = graph.nodes.len() * graph.nodes.len();
        let mut min_a = Vec::new();
        let mut min_b = Vec::new();
        let mut curr_graph = Graph::new();
        for _i in 0..graph.nodes.len() * graph.nodes.len() {
            // for _ in 0..1 {
            curr_graph = graph.clone();
            let (cut, a, b) = curr_graph.min_cut(&mut rng);
            if cut < min {
                min = cut;
                min_a = a;
                min_b = b;
            }
        }
        dbg!(min, min_a, &min_b);
        // Print edges of the remaining two nodes.
        // Filter non-removed nodes by only keeping the ones that have non-zero edges still.
        let mut the_two = Vec::new();
        for node in &curr_graph.nodes {
            if node.edges.len() > 0 {
                the_two.push(node);
            }
        }
        dbg!("The two", the_two);

        assert_eq!(min, 2);
    }
}

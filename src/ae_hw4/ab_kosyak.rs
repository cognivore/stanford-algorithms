crate::entry_point!("kosyak", main);

use crate::aa_kosajaru::kosaraju;
use crate::graph::Graph;

pub fn main() {
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
    dbg!(&sccs);
    assert_eq!(sccs.len(), 1);
}

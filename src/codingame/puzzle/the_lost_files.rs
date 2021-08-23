use std::{
    collections::{BTreeSet, HashMap},
    hash::Hash,
};

use crate::util::*;

crate::entry_point!("codingame/puzzle/the_lost_files/1", first_attempt);

pub fn first_attempt() {
    let w = load_problem(1);
    //eprintln!("{:?}", w);
    let g = world_to_adj_list_graph(w);
    //eprintln!("{:#?}", g);
    let (Continents(c), Tiles(t)) = solve(load_problem(1));
    println!("{} {}", c, t);
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct World {
    // vertices: Vec<Vertex>, <- for flavor points
    edges: Vec<Edge>,
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Edge(VertexId, VertexId);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Continents(i32);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Tiles(i32);

type Graph = HashMap<VertexId, Vec<VertexId>>;

pub fn load_problem(id: i32) -> World {
    let mut res = Vec::new();
    let path = project_path(format!("./data/codingame/puzzle/the_lost_files/{}.in", id));
    let data0 = std::fs::read_to_string(path).unwrap();
    let mut data = data0.split('\n');
    data.next();
    for l in data {
        if l == "" {
            continue;
        };
        let mut vals = l.split(' ');
        let from = VertexId((vals.next().unwrap()).parse::<i32>().unwrap());
        let to = VertexId((vals.next().unwrap()).parse::<i32>().unwrap());
        res.push(Edge(from, to));
    }
    World { edges: res }
}

pub fn world_to_adj_list_graph(w: World) -> Graph {
    let mut res: Graph = HashMap::new();

    for Edge(from, to) in w.edges {
        mut_register_edge(&mut res, from, to);
        mut_register_edge(&mut res, to, from);
    }
    res
}

pub fn mut_register_edge(g: &mut Graph, from: VertexId, to: VertexId) {
    if let Some(adj_list) = (*g).get_mut(&from) {
        adj_list.push(to);
    } else {
        let mut adj_list = Vec::new();
        adj_list.push(to);
        (*g).insert(from, adj_list);
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Level(i32);

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct VertexId(i32);

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct SolverState {}

pub fn solve(w: World) -> (Continents, Tiles) {
    panic!("not implemented")
}

//// I ended up not needing this, but check out ./src/howto/raw_entry_api.rs for relevant howto!
// pub fn mut_pop_map<K: Hash + Eq + Clone, V>(m: &mut HashMap<K, V>) -> Option<(K, V)> {
//     if let Some(k) = m.keys().next().cloned() {
//         m.remove_entry(&k)
//     } else {
//         None
//     }
// }

#[test]
fn test_p1_passes() {
    let w = load_problem(1);
    assert_eq!((Continents(2), Tiles(4)), solve(w));
}

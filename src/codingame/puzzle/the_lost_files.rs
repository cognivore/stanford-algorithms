use std::{
    collections::{BTreeMap, HashMap, VecDeque},
    hash::Hash,
    iter::FromIterator,
};

use std::convert::TryFrom;

use crate::util::*;

crate::entry_point!("codingame/puzzle/the_lost_files/1", fifth_attempt);

pub fn fifth_attempt() {
    let w = load_problem(1);
    let _g = world_to_adj_list_graph(w);
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

type VertexLevels = BTreeMap<VertexId, Level>;

#[derive(Debug, PartialEq, Eq, Clone)]
struct LoopWalk {
    tip: VertexId,
    level: i32,
    path: Vec<VertexId>,
    is_frontier_loaded: bool,
    frontier: VecDeque<(Vec<VertexId>, Level)>,
    visited: VertexLevels,
    found: Vec<Vec<VertexId>>,
}

fn find_loops(g: &Graph, walk: &LoopWalk) -> Vec<(VertexId, Level)> {
    let mut ascend: usize = 0;
    let l = walk.path.len();
    let mut ret = vec![];
    for curr_v in &walk.path {
        if l - ascend < 3 {
            return ret.clone();
        }
        let adj = g.get(&curr_v).unwrap();
        if adj.contains(&walk.tip) {
            ret.push((curr_v.clone(), Level(i32::try_from(ascend).unwrap())))
        }
        ascend = ascend + 1;
    }
    ret
}

fn mk_walk(
    walk: LoopWalk,
    path: Vec<VertexId>,
    level: Level,
    frontier: VecDeque<(Vec<VertexId>, Level)>,
) -> LoopWalk {
    LoopWalk {
        tip: path.last().unwrap().clone(),
        level: level.0,
        path: path,
        frontier: frontier,
        ..walk
    }
}

fn solve_do(g: &Graph, walk: LoopWalk) -> LoopWalk {
    // Entered new tip, or deferred to a visited node. Let's differentiate!
    if walk.visited.contains_key(&walk.tip) {
        let mut frontier1 = walk.frontier.clone();
        return match frontier1.pop_front() {
            None => return walk,
            Some((new_path, new_level)) => {
                solve_do(g, mk_walk(walk, new_path, new_level, frontier1.clone()))
            }
        };
    }
    let mut walk1 = walk.clone();
    // Set visited
    assert_eq!(walk1.visited.insert(walk.tip, Level(walk.level + 1)), None);
    // Find loops
    let new_loops = find_loops(g, &walk);
    for (_connector, _lvl) in new_loops {
        walk1.found.push(walk.path.clone());
    }
    // Handle non-visited connections
    if let Some(cs) = g.get(&walk.tip) {
        for c in cs {
            if !(walk.visited.contains_key(c)) {
                let mut curr_path = walk.path.clone();
                curr_path.push(c.clone());
                let fr = (curr_path, Level(walk.level + 1));
                walk1.frontier.push_front(fr);
            }
        }
    }
    // Update tip and path to reflect the topmost frontier element
    match walk1.frontier.pop_front() {
        None => walk1,
        Some((new_path, new_level)) => solve_do(
            g,
            mk_walk(walk1.clone(), new_path, new_level, walk1.frontier.clone()),
        ),
    }
}

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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct Level(i32);

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct VertexId(i32);

pub fn solve(w: World) -> (Continents, Tiles) {
    let g = world_to_adj_list_graph(w);
    let any_entry = &VertexId(-1); // We initialise to nonsense to crash on bug
    let mut acc = (Continents(0), Tiles(0));
    let mut walk = LoopWalk {
        tip: any_entry.clone(),
        level: 0,
        path: vec![any_entry.clone()],
        is_frontier_loaded: false,
        frontier: VecDeque::from_iter(vec![]),
        visited: BTreeMap::new(),
        found: Vec::new(),
    };
    for x in g.keys() {
        let visited1 = walk.visited.clone();
        if visited1.contains_key(&x) {
            continue;
        } else {
            walk = solve_do(
                &g,
                LoopWalk {
                    tip: x.clone(),
                    path: vec![x.clone()],
                    ..walk
                },
            );
            let (Continents(c), Tiles(t)) = acc.clone();
            acc = (
                Continents(c + 1),
                Tiles(t + i32::try_from(walk.found.len()).unwrap()),
            );
            walk = LoopWalk {
                tip: x.clone(),
                level: 0,
                path: vec![x.clone()],
                found: Vec::new(),
                ..walk
            };
        }
    }

    acc.clone()
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

use crate::util::*;

crate::entry_point!("codingame/puzzle/the_lost_files", main);

pub fn main() {
    let w = load_problem(1);
    eprintln!("{:?}", w);
    let (Continents(c), Tiles(t)) = solve(load_problem(1));
    println!("{} {}", c, t);
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct World {
    // vertices: Vec<Vertex>, <- for flavor points
    edges: Vec<Edge>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Edge(i32, i32);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Continents(i32);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Tiles(i32);

pub fn load_problem(id: i32) -> World {
    let mut res = Vec::new();
    let path = project_path(format!("./data/codingame/puzzle/the_lost_files/{}.in", id));
    let data0 = std::fs::read_to_string(path).unwrap();
    let mut data = data0.split('\n');
    data.next();
    for l in data {
        if l == "" {continue};
        let mut vals = l.split(' ');
        let from = (vals.next().unwrap()).parse::<i32>().unwrap();
        let to = (vals.next().unwrap()).parse::<i32>().unwrap();
        res.push(Edge(from, to));
    }
    World {edges: res}
}

fn render_world(w: World) -> String {
    panic!("Not implemented")
}

fn solve(w: World) -> (Continents, Tiles) {
    panic!("Not implemented")
}

#[test]
fn test_p1_passes() {
    let w = load_problem(1);
    assert_eq!((Continents(2), Tiles(4)), solve(w));
}

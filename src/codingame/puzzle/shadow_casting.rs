use std::cmp::max;

use crate::util::*;

crate::entry_point!("codingame/puzzle/shadow_casting", main);

type World = Vec<Vec<Cell>>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Cell {
    Empty,
    LightShadow,
    DeepShadow,
    Rock,
}

// This can have a buffered verison for a true one-pass solution
pub fn load_problem(id: i32) -> World {
    let path = project_path(format!("./data/codingame/puzzle/shadow_casting/{}.in", id));
    let data0 = std::fs::read_to_string(path).unwrap();
    let mut data = data0.split('\n');
    let mut res = Vec::new();
    data.next();
    for l in data {
        let mut lw = Vec::new();
        for c in l.chars() {
            lw.push(char_to_cell(c));
        }
        res.push(lw);
    }
    res
}

pub fn char_to_cell(c: char) -> Cell {
    match c {
        '#' => Cell::Rock,
        '-' => Cell::DeepShadow,
        '`' => Cell::LightShadow,
        ' ' => Cell::Empty,
        _ => panic!("{} unsupported", c),
    }
}

pub fn cell_to_char(c: Cell) -> char {
    match c {
        Cell::Rock => '#',
        Cell::DeepShadow => '-',
        Cell::LightShadow => '`',
        Cell::Empty => ' ',
    }
}

pub fn merge_worlds<'a>(w0: &'a mut World, w1: &'a mut World) -> World {
    if w0.len() >= w1.len() {
        merge_larger_world_with_smaller_world(w0, w1);
        return (*w0.clone()).to_vec();
    } else {
        merge_larger_world_with_smaller_world(w1, w0);
        return (*w1.clone()).to_vec();
    }
}

fn merge_larger_world_with_smaller_world<'a>(w0: &'a mut World, w1: &'a World) {
    // keep |w0| - |w1| and merge |w1|
    let mut w0_iter = w0.iter_mut();
    let mut w1_iter = w1.iter();
    for _x in 0..(w0_iter.len() - w1_iter.len()) {
        w0_iter.next();
    }
    for cells0 in w0_iter {
        if let Some(cells1) = w1_iter.next() {
            merge_cells(cells0, cells1);
        } else {
            return;
        }
    }
}

fn merge_cells(cells0: &mut Vec<Cell>, cells1: &Vec<Cell>) {
    let c0l = cells0.len();
    let c1l = cells1.len();
    if c0l > c1l {
        for i in 0..c0l {
            if c1l > i {
                cells0[i] = max(cells0[i], cells1[i]);
            }
        }
    } else {
        for i in 0..c1l {
            if c0l > i {
                cells0[i] = max(cells0[i], cells1[i]);
            } else {
                cells0.push(cells1[i]);
            }
        }
    }
}

pub fn mk_shadow(world_line: Vec<Cell>) -> World {
    let mut res = Vec::new();
    let rocks = Vec::new();
    res.push(rocks);
    let mut deep = Vec::new();
    deep.push(Cell::Empty);
    res.push(deep);
    let mut light = Vec::new();
    light.push(Cell::Empty);
    light.push(Cell::Empty);
    res.push(light);
    for c in world_line {
        res[0].push(c);
        match c {
            Cell::Rock => {
                res[1].push(Cell::DeepShadow);
                res[2].push(Cell::LightShadow);
            }
            _ => {
                res[1].push(Cell::Empty);
                res[2].push(Cell::Empty);
            }
        }
    }
    return res;
}

pub fn main() {
    //TODO: Arguments
    let problem = load_problem(1);
    let mut res = Vec::new();
    res.push(Vec::new());
    res.push(Vec::new());
    res.push(Vec::new());
    for x in problem {
        let mut shadow = mk_shadow(x.clone());
        merge_worlds(&mut res, &mut shadow);
        res.push(Vec::new());
    }
    println!("{}", render_world(res));
}

pub fn render_world(w: World) -> String {
    let mut res = Vec::new();
    for cc in w {
        let mut l = String::new();
        for c in cc {
            l.push(cell_to_char(c))
        }
        res.push(l);
    }
    res.join("\n")
}

#[test]
fn world_is_ordered() {
    assert!(Cell::Rock > Cell::LightShadow);
}

#[test]
fn x_parses() {
    let w = load_problem(1);
    assert_eq!(
        vec![
            Cell::Empty,
            Cell::Empty,
            Cell::Rock,
            Cell::Empty,
            Cell::Empty,
            Cell::Empty,
            Cell::Empty,
            Cell::Empty,
            Cell::Rock
        ],
        w[0]
    );
    assert_eq!(
        "  #     #
   #   #
    # #
     #
    # #
   #   #
  #     #",
        render_world(w)
    );
}

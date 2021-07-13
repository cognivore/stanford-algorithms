/*

  In the actual submission there was a test case that used ` character as a rock.
  That's why we had to quickly accomodate for this rules extension by introducing

    Cell::Ext(char)

  part of the enum, and changing the logic of creating initial world from the problem.

  We also had a bug where there was a dangling empty vector.
  We work-arounded it by popping the last element of the result vector in main1.


*/
use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

use std::cmp::max;



type World = Vec<Vec<Cell>>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Cell {
    Empty,
    LightShadow,
    DeepShadow,
    Rock,
    Ext(char),
}


pub fn char_to_cell(c: char) -> Cell {
    match c {
        '#' => Cell::Rock,
        '-' => Cell::DeepShadow,
        '`' => Cell::LightShadow,
        ' ' => Cell::Empty,
        _ => Cell::Ext(c),
    }
}

pub fn cell_to_char(c: Cell) -> char {
    match c {
        Cell::Ext(x) => x,
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
            Cell::Ext(c) => {
                res[1].push(Cell::DeepShadow);
                res[2].push(Cell::LightShadow);
            }
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

pub fn main1(problem: World) {
    //TODO: Arguments
    let mut res = Vec::new();
    res.push(Vec::new());
    res.push(Vec::new());
    res.push(Vec::new());
    for x in problem {
        let mut shadow = mk_shadow(x.clone());
        merge_worlds(&mut res, &mut shadow);
        res.push(Vec::new());
    }
    let zz = res.pop();
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

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);
    let mut ls: Vec<String> = Vec::new();
    let mut res = Vec::new();
    for i in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let popped = input_line.pop().unwrap();
        if popped != '\n' {
            input_line.push(popped);
        }
        let mut lw = Vec::new();
        for c in input_line.chars() {
            match c {
                ' ' => lw.push(Cell::Empty),
                _ => lw.push(Cell::Ext(c)),
            }
        }
        res.push(lw);
    }
    //println!("{}", render_world(res.clone()));
    main1(res);
}
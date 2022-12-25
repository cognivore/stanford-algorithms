// A six-sided die, with numbers written on each of its faces, is placed on the 6-by-6 grid above, in the lower-left (yellow) corner. It then makes a sequence of “moves”. Each move consists of tipping the die into an orthogonally adjacent square within the grid.
//
// The die starts with a “score” of 0. On the Nth move, its score increases by N times the value of the die facing up after the move. However, the die is only allowed to move into a square if its score after the move matches the value in the square. Also, the die cannot be translated or rotated in place in addition to these moves.
//
// After some number of moves the die arrives in the upper-right (blue) corner.
//
// The answer to this puzzle is the sum of values in the unvisited squares from the die’s journey.

crate::entry_point!("janestreet/die-agony", main);

// Function that returns grid.
pub fn grid_spec() -> Vec<Vec<i32>> {
    vec![
        vec![57, 33, 132, 268, 492, 732],
        vec![81, 123, 240, 443, 353, 508],
        vec![186, 42, 195, 704, 452, 228],
        vec![-7, 2, 357, 452, 317, 395],
        vec![5, 23, -4, 592, 445, 620],
        vec![0, 77, 32, 403, 337, 452],
    ]
}

pub fn grid() -> Vec<Vec<i32>> {
    vec![
        vec![0, 77, 32, 403, 337, 452],
        vec![5, 23, -4, 592, 445, 620],
        vec![-7, 2, 357, 452, 317, 395],
        vec![186, 42, 195, 704, 452, 228],
        vec![81, 123, 240, 443, 353, 508],
        vec![57, 33, 132, 268, 492, 732],
    ]
}

// Die is a 6-sided die that can be moved around a grid.
// Here's class:
#[derive(Debug, Clone)]
struct Die {
    grid: Vec<Vec<i32>>,
    x: usize,
    y: usize,
    face: i32,
    score: i32,
    known_faces: Vec<i32>,
    squares_visited: Vec<(usize, usize)>,
}

impl Die {
    // Constructor for Die.
    fn new(grid: Vec<Vec<i32>>) -> Die {
        Die {
            grid,
            x: 0,
            y: 0,
            face: -1, // -1 means that we don't know the face yet.
            score: 0,
            known_faces: vec![],
            squares_visited: vec![],
        }
    }
    // Position of the die.
    fn position(&self) -> (usize, usize) {
        (self.x, self.y)
    }
    // Check if the die is at the goal.
    fn is_at_goal(&self) -> bool {
        // We're not using magic numbers here, checking that we're at the top right corner.
        self.x == self.grid.len() - 1 && self.y == self.grid[0].len() - 1
    }

    // Get orthogonally adjacent squares.
    fn adjacent(&self) -> Vec<(usize, usize)> {
        let mut adjacent = vec![];
        if self.x > 0 {
            adjacent.push((self.x - 1, self.y));
        }
        if self.x < self.grid.len() - 1 {
            adjacent.push((self.x + 1, self.y));
        }
        if self.y > 0 {
            adjacent.push((self.x, self.y - 1));
        }
        if self.y < self.grid[0].len() - 1 {
            adjacent.push((self.x, self.y + 1));
        }
        adjacent
    }

    // Move the die to target x, y (tx, ty).
    // To do that we first reduce tx,ty square's value by the current score.
    // Then we divide the difference by the number of moves we've made.
    // This gives us the value of the new face of the die.
    // Then we check if the new face is already known.
    // If not, we check if there are already six known faces.
    // If there are, the move is illegal and we return false.
    // If there aren't, we record that face in the list of known faces.
    // Then we move the die to the new position.
    // Finally, we update the score.
    fn move_to(&mut self, tx: usize, ty: usize) -> bool {
        let target = self.grid[ty][tx];
        let new_face_helper = target - self.score;
        let new_face =
            // Check for length of squares visited to avoid division by zero.
            if self.squares_visited.len() == 0 {
                new_face_helper
            } else {
                new_face_helper / self.squares_visited.len() as i32
            };
        if self.known_faces.contains(&new_face) {
            return false;
        }
        if self.known_faces.len() == 6 {
            return false;
        }
        self.known_faces.push(new_face);
        //self.move_by(tx as i32 - self.x as i32, ty as i32 - self.y as i32);
        self.x = tx;
        self.y = ty;
        self.squares_visited.push((tx, ty));
        self.score = target;
        true
    }
}

// Breadth-first search, where the die starts with face -1 and then we try all possible moves.
// While doing so, we keep track of the faces that we have chosen in the `known_faces` field.
// We also keep track of the squares visited in every arm of the breadth-first search.
// If we reach the goal, we return the Die object with the correct face and squares visited.
// If there is no possible move, we terminate the arm of the breadth-first search.
fn bfs(die: Die) -> Option<Die> {
    let mut queue = vec![die];
    while !queue.is_empty() {
        let die = queue.remove(0);
        if die.is_at_goal() {
            return Some(die);
        }
        for (x, y) in die.adjacent() {
            // Check if we have visited it already. If so, continue.
            if die.squares_visited.contains(&(x, y)) {
                continue;
            }
            println!("x: {}, y: {}", x, y);
            let target = die.grid[y][x];
            if die.known_faces.contains(&target) {
                continue;
            }
            let mut new_die = die.clone();
            // Move the new_die to the new position. If it's not possible, terminate the arm.
            if !new_die.move_to(x, y) {
                continue;
            }
            queue.push(new_die);
        }
    }
    None
}

pub fn main() {
    // Run breadth-first search to find a path to the goal.
    let result_die_maybe = bfs(Die::new(grid()));
    print!("{:?}", result_die_maybe.as_ref().unwrap().known_faces);
    print!("{:?}", result_die_maybe.as_ref().unwrap().squares_visited);
    // Get the sum of the squares that aren't visited.
    let score = grid()
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(x, _)| {
                    !result_die_maybe
                        .as_ref()
                        .unwrap()
                        .squares_visited
                        .contains(&(*x, y))
                })
                .map(|(_, v)| *v)
                .sum::<i32>()
        })
        .sum::<i32>();
    println!("{}", score);
}

// Test that if the die is at 0,0 there are just 2 possible moves.
#[test]
fn test_adjacent_at_start() {
    let die = Die::new(grid());
    assert_eq!(die.adjacent().len(), 2);
}

// Test that if the die is at 5,5 it is at the goal.
#[test]
fn test_is_at_goal() {
    let mut die = Die::new(grid());
    die.x = 5;
    die.y = 5;
    assert!(die.is_at_goal());
}

// Test that if the die is at 5,5 there are just 2 possible moves.
#[test]
fn test_adjacent_at_goal() {
    let mut die = Die::new(grid());
    die.x = 5;
    die.y = 5;
    assert_eq!(die.adjacent().len(), 2);
}

// Test that if the die is at 0,0 it is not at the goal.
#[test]
fn test_is_not_at_goal() {
    let die = Die::new(grid());
    assert!(!die.is_at_goal());
}

// Test that if the die is at 3,5, there are 3 possible moves.
#[test]
fn test_adjacent_at_3_5() {
    let mut die = Die::new(grid());
    die.x = 3;
    die.y = 5;
    assert_eq!(die.adjacent().len(), 3);
}

// Test that if the die is at 1,1, there are 4 possible moves.
#[test]
fn test_adjacent_at_1_1() {
    let mut die = Die::new(grid());
    die.x = 1;
    die.y = 1;
    assert_eq!(die.adjacent().len(), 4);
}

// In this tiny 3x3 grid, the following happens:
// Die starts at 0,0 with value 0 then goes right to 8.
// On move number 2, the face facing up shows 0. The score is 8 + 2*0 = 8. This is why it goes to 8 again.
// On move number 3, the face facing up shows 8. The score is 8 + 2*0 + 3*8 = 32. This is why it goes to square with value 32.
// On move number 4, the face facing up shows 13. The score is 8 + 2*0 + 3*8 + 4*13 = 84. This is why it goes to square with value 84.
// On move number 5, the face facing up shows 1. The score is 8 + 2*0 + 3*8 + 4*13 + 5*1 = 89. This is why it goes to square with value 89.
// Finally, on move number 6, the face facing up shows 69. The score is 8 + 2*0 + 3*8 + 4*13 + 5*1 + 6*69 = 503. This is why it goes to square with value 503.
pub fn tiny_grid() -> Vec<Vec<i32>> {
    vec![
        vec![0, 8, 8],     // L1
        vec![-13, 84, 32], // L2
        vec![0, 89, 503],  // L3
    ]
}

// Test that in the tiny grid, the die goes to the correct squares.
#[test]
fn test_tiny_grid() {
    let result_die_maybe = bfs(Die::new(tiny_grid()));
    print!("{:?}", result_die_maybe.as_ref().unwrap().known_faces);
    assert_eq!(
        result_die_maybe.unwrap().squares_visited,
        // Check that we went right, right, up, left, up, right.
        vec![(0, 0), (0, 1), (0, 2), (1, 2), (1, 1), (2, 1), (2, 2)]
    );
}

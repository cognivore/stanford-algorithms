trait SelectSortable {
    fn select(&mut self);
}

impl SelectSortable for Vec<u64> {
    fn select(&mut self) {
        for start in 0..self.len() {
            let mut min = (self[start], start);
            for i in start..self.len() {
                if self[i] < min.0 {
                    min = (self[i], i);
                }
            }
            self.swap(start, min.1);
        }
    }
}

/* Will get you in trouble with borrow checker:
fn select(mut x: Vec<u64>) -> () {
    for start in 0..x.len() {
        let mut min = (x[start], start);
        for i in start..x.len() {
            if x[i] < min.0 {
                min = (x[i], i);
            }
        }
        swap(&mut x, start, min.1);
    }
}

fn swap(x: &mut Vec<u64>, from: usize, to: usize) -> () {
    let to_val = x[to];
    x[to] = x[from];
    x[from] = to_val;
}
*/

// Tests!

#[cfg(test)]
mod tests {
    use rand::seq::SliceRandom;

    use super::*;

    #[test]
    fn test_shuffle() {
        let mut xs = vec![];
        for x in 0..=10 {
            xs.push(x);
            xs.push(x);
        }
        let mut rng = rand::thread_rng();
        let image = xs.clone();
        xs.shuffle(&mut rng);
        xs.select();
        // Assert that select(xs) mutates it to match image:
        assert_eq!(xs, image);
    }
}

crate::entry_point!("median_", main);
// Binaryheap
use std::collections::BinaryHeap;

// Min heap stores i64 with inverted Ord
#[derive(Eq, PartialEq, Debug)]
struct MinInt(i64);
impl PartialOrd for MinInt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.0.partial_cmp(&self.0)
    }
}
impl Ord for MinInt {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.cmp(&self.0)
    }
}

pub fn main() {
    // Create Min-heap and Max-heap
    let mut high: BinaryHeap<MinInt> = BinaryHeap::new();
    let mut low: BinaryHeap<i64> = BinaryHeap::new();
    let mut medians_over_time = Vec::new();
    // Stream ints from file
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let file = File::open("./data/Median.txt").unwrap();
    let reader = BufReader::new(file);
    let mut i = 0;
    for line in reader.lines() {
        i += 1;
        // Diagnostics every 100 iterations
        if i % 100 == 0 {
            println!(
                "{:?}@{} {:?}@{}",
                &low.peek(),
                &low.len(),
                &high.peek(),
                &high.len()
            )
        }
        let line = line.unwrap();
        let num: i64 = line.parse().unwrap();
        let low_len = low.len();
        let high_len = high.len();
        // When we insert a new number into low, we need to move the max of low into high
        // And vice versa.
        if low_len == 0 || num < *low.peek().unwrap() {
            low.push(num);
            if high_len < low_len {
                high.push(MinInt(low.pop().unwrap()));
            }
        } else {
            high.push(MinInt(num));
            if low_len < high_len {
                low.push(high.pop().unwrap().0);
            }
        }
        let k = if i % 2 == 0 {
            // Even
            i / 2
        } else {
            // Odd
            (i + 1) / 2
        };
        if low.len() >= k {
            medians_over_time.push(*low.peek().unwrap());
        } else {
            medians_over_time.push(high.peek().unwrap().0);
        }
    }
    // Return sum of medians mod 10000
    let sum: i64 = medians_over_time.iter().sum();
    println!("{}", sum % 10000);
}

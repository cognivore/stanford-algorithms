// fxhash hashset
use fxhash::FxHashSet as HashSet;
crate::entry_point!("2sum_", main);

pub fn main() {
    let mut distinct = 0;
    let hashset = read_hashset_from_file();
    for i in -10000..=10000 {
        // Diagnostics every 100 iterations
        if i % 500 == 0 {
            println!("{}", i);
        }
        let mut count = 0;
        for x in hashset.iter() {
            let y = i - x;
            if x != &y && hashset.contains(&y) {
                count += 1;
            }
        }
        if count == 1 {
            distinct += 1;
        }
    }
    println!("{}", distinct);
}

/*
# Input sample from ./data/algo1-programming_prob-2sum.txt
68037543430
-21123414637
56619844751
*/
pub fn read_hashset_from_file() -> HashSet<i64> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let file = File::open("./data/algo1-programming_prob-2sum.txt").unwrap();
    let reader = BufReader::new(file);
    let mut hashset = HashSet::default();
    for line in reader.lines() {
        let line = line.unwrap();
        let num: i64 = line.parse().unwrap();
        hashset.insert(num);
    }
    hashset
}

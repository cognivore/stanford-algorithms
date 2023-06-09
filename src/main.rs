#![feature(hash_raw_entry)]

mod example;
mod graph;
mod util;

#[path = "./howto/raw_entry_api.rs"]
mod raw_entry_api;

#[path = "./howto/custom_array.rs"]
mod custom_array;

#[path = "./aa_introduction/aa_dcmul.rs"]
mod aa_dcmul;

#[path = "./aa_introduction/ab_sort.rs"]
mod ab_sort;

#[path = "./aa_introduction/ac_merge.rs"]
mod ac_merge;

#[path = "./ab_hw1/aa_inversions.rs"]
mod aa_inversions;

#[path = "./ac_hw2/quick.rs"]
mod quick;

#[path = "./ad_hw3/aa_karger.rs"]
mod aa_karger;

#[path = "./ae_hw4/aa_kosajaru.rs"]
mod aa_kosajaru;

#[path = "./ae_hw4/ab_kosyak.rs"]
mod ab_kosyak;

#[path = "./af_hw5_dijkstra/aa_dijkstra.rs"]
mod aa_dijkstra;

#[path = "./ag_hw6/aa_two_sum.rs"]
mod aa_two_sum;

#[path = "./ag_hw6/ab_median.rs"]
mod ab_median;

#[linkme::distributed_slice]
static ENTRY_POINTS: [(&'static str, fn())] = [..];

/**
Entry points can be defined like this:
```
crate::entry_point!("hello", hello);
fn hello() { ... }
```

If you define more than one entry point in one module,
for technical reasons you need to specify unique identifier names:
```
crate::entry_point!("hello1", hello1, _EP_HELLO1);
fn hello1() { ... }

crate::entry_point!("hello2", hello2, _EP_HELLO2);
fn hello2() { ... }
```
*/

#[macro_export]
macro_rules! entry_point {
    ($name:expr, $f:expr) => {
        $crate::entry_point!($name, $f, _ENTRY_POINT);
    };
    ($name:expr, $f:expr, $static_name:ident) => {
        #[linkme::distributed_slice($crate::ENTRY_POINTS)]
        static $static_name: (&'static str, fn()) = ($name, $f);
    };
}

fn ensure_entry_points_unique() {
    for (i, (name, _)) in ENTRY_POINTS.iter().enumerate() {
        for (name2, _) in &ENTRY_POINTS[..i] {
            assert_ne!(name, name2, "duplicate entry point names");
        }
    }
}

#[test]
fn entry_points_unique() {
    ensure_entry_points_unique();
}

fn main() {
    ensure_entry_points_unique();

    if let Some(entry_point) = std::env::args().nth(1) {
        let p = ENTRY_POINTS.iter().find(|(name, _)| name == &entry_point);
        if let Some((_, f)) = p {
            f();
            return;
        } else {
            eprintln!("no entry point {:?}", entry_point);
        }
    } else {
        eprintln!("entry point not specified");
        eprintln!("usage:");
        eprintln!("  cargo run <entry point>");
    }
    eprintln!("possible entry points:");
    for (name, _) in ENTRY_POINTS {
        eprintln!("- {}", name);
    }
    std::process::exit(1);
}

use std::u128;

crate::entry_point!("comparisons", main);

#[derive(Debug, Clone, Copy)]
enum Pivot {
    First,
    Last,
    Median,
}

fn quick_sort(input: &mut [u64], pivot: Pivot) -> u128 {
    let mut comparisons = 0;
    let mut input = input;
    quick_sort_inner(&mut input, &mut comparisons, pivot);
    comparisons
}

fn quick_sort_inner(input: &mut [u64], comparisons: &mut u128, pivot: Pivot) {
    let len = input.len();
    if len < 2 {
        return;
    }
    let pivot_index = match pivot {
        Pivot::First => 0,
        Pivot::Last => len - 1,
        Pivot::Median => {
            let first = input[0];
            let last = input[len - 1];

            let mid_i = len / 2 - (len % 2 == 0) as usize;
            // If len is even, we want the first of the two middle elements.
            let mid = input[mid_i];
            let mut candidates = [first, mid, last];
            quick_sort(&mut candidates, Pivot::First);
            if candidates[1] == first {
                0
            } else if candidates[1] == mid {
                mid_i
            } else {
                len - 1
            }
        }
    };

    /*
    dbg!(
        "Chose {?:} index: {}, value: {}",
        pivot,
        pivot_index,
        input[pivot_index],
        "among",
        input.to_vec()
    );
    */

    input.swap(0, pivot_index);
    let mut last_smaller = 0;
    for i in 1..len {
        if input[i] < input[0] {
            last_smaller += 1;
            input.swap(i, last_smaller);
        }
    }
    input.swap(0, last_smaller);
    *comparisons += (len - 1) as u128;
    quick_sort_inner(&mut input[0..last_smaller], comparisons, pivot);
    quick_sort_inner(&mut input[last_smaller + 1..len], comparisons, pivot);
}

pub fn main() {
    // Read the input into a vector of u64 integers.
    // Input is in ./data/QuickSort.txt
    let input: Vec<u64> = std::fs::read_to_string("./data/QuickSort.txt")
        .expect("Something went wrong reading the file")
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let comparisons_first = quick_sort(&mut input.clone(), Pivot::First);
    let comparisons_last = quick_sort(&mut input.clone(), Pivot::Last);
    let comparisons_median = quick_sort(&mut input.clone(), Pivot::Median);

    println!(
        "Comparisons first: {}, last: {}, median: {}",
        comparisons_first, comparisons_last, comparisons_median
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::seq::SliceRandom;

    #[test]
    fn test_comparisons() {
        let input = vec![3, 8, 2, 5, 1, 4, 7, 6];
        let mut output1 = input.clone();
        let mut output2 = input.clone();
        let mut output3 = input.clone();
        let comparisons_first = quick_sort(&mut output1, Pivot::First);
        let comparisons_last = quick_sort(&mut output2, Pivot::Last);
        let comparisons_median = quick_sort(&mut output3, Pivot::Median);
        assert_eq!(comparisons_first, 15);
        assert_eq!(comparisons_last, 15);
        assert_eq!(comparisons_median, 13);
        assert_eq!(output1, vec![1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(output2, vec![1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(output3, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_sanity() {
        // Smallest possible non-trivial input:
        let input = vec![1, 2, 0];
        let mut output1 = input.clone();
        let mut output2 = input.clone();
        let mut output3 = input.clone();
        let comparisons_first = quick_sort(&mut output1, Pivot::First);
        let comparisons_last = quick_sort(&mut output2, Pivot::Last);
        let comparisons_median = quick_sort(&mut output3, Pivot::Median);
        assert_eq!(comparisons_first, 2);
        assert_eq!(comparisons_last, 3);
        assert_eq!(comparisons_median, 2);
        assert_eq!(output1, vec![0, 1, 2]);
        assert_eq!(output2, vec![0, 1, 2]);
        assert_eq!(output3, vec![0, 1, 2]);
    }

    #[test]
    fn random_with_duplicates() {
        // Shuffle the input
        let mut input = vec![1, 1, 8, 4, 9, 6, 7, 9, 8, 8];
        input.shuffle(&mut rand::thread_rng());

        let mut output1 = input.clone();
        let mut output2 = input.clone();
        let mut output3 = input.clone();

        quick_sort(&mut output1, Pivot::First);
        quick_sort(&mut output2, Pivot::Last);
        quick_sort(&mut output3, Pivot::Median);

        assert_eq!(output1, vec![1, 1, 4, 6, 7, 8, 8, 8, 9, 9]);
        assert_eq!(output2, vec![1, 1, 4, 6, 7, 8, 8, 8, 9, 9]);
        assert_eq!(output3, vec![1, 1, 4, 6, 7, 8, 8, 8, 9, 9]);
    }
}

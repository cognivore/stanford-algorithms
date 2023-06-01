use std::u128;

crate::entry_point!("inversions", main);

// Take the ownership to mutate!
// See https://zulip.memorici.de/#narrow/stream/4-rust/topic/Vectors.2C.20slices.2C.20unsized.20types.20and.20fat.20pointers/near/59301
fn count_inversions(mut xs: Vec<u64>) -> u128 {
    sort_and_count_inversions_do(&mut xs)
}

fn sort_and_count_inversions_do(xs: &mut [u64]) -> u128 {
    let len = xs.len();
    let mid = len / 2;
    if len < 2 {
        return 0;
    }
    let left_inversions = sort_and_count_inversions_do(&mut xs[..mid]);
    let right_inversions = sort_and_count_inversions_do(&mut xs[mid..]);
    let mut split_inversions: u128 = 0;
    let mut li = 0;
    let mut ri = mid;
    let mut res = Vec::with_capacity(len);
    while li < mid && ri < len {
        if xs[li] < xs[ri] {
            res.push(xs[li]);
            li += 1;
        } else {
            res.push(xs[ri]);
            ri += 1;
            split_inversions += (mid - li) as u128;
            // If we're at 0 out of 3, then we have 3 - 0 = 3 inversions.
            // ...
            // If we're at 2 out of 3 (meaning that we're yet to process the last element), then we have 3 - 2 = 1 inversion.
            // When we have processed the last element, the while loop will terminate and we won't get here.
        }
    }
    // Copy the rest of the left or right side. We don't need to increase split inversion counter
    // because even if there is some stuff left in li, our "heuristic" of adding (mid - li) has
    // already taken care of that.
    if li < mid {
        res.extend_from_slice(&xs[li..mid]);
    } else if ri < len {
        res.extend_from_slice(&xs[ri..]);
    }

    //println!("Length of xs: {} ~ length of res: {}", xs.len(), res.len());
    // Rewrite xs with the sorted result.
    xs.copy_from_slice(&res);
    left_inversions + right_inversions + split_inversions
}

pub fn main() {
    // Read the input into a vector of u64 integers.
    // Input is in ./data/IntegerArray.txt
    let input: Vec<u64> = std::fs::read_to_string("./data/IntegerArray.txt")
        .expect("Something went wrong reading the file")
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    println!(
        "Input length: {}; max: {}, min: {}, average: {}",
        input.len(),
        input.iter().max().unwrap(),
        input.iter().min().unwrap(),
        input.iter().sum::<u64>() as f64 / input.len() as f64,
        //count_inversions(input.clone()) // Tidbit. println macro won't release borrow on
        //input (duh). So we need another println! or clone() to release the borrow.
    );
    // Tidbit.
    // Note how we have declared input as immutable, but since we forfeit the ownership to the
    // underlying function and we can't use it anymore, the underlying function can declare its
    // argument as mutable.
    println!("Inversions: {}", count_inversions(input));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_inversions() {
        assert_eq!(super::count_inversions(vec![1, 3, 5, 2, 4, 6]), 3);
        assert_eq!(super::count_inversions(vec![1, 5, 3, 2, 4]), 4);
        assert_eq!(super::count_inversions(vec![5, 4, 3, 2, 1]), 10);
        assert_eq!(super::count_inversions(vec![1, 6, 3, 2, 4, 5]), 5);
        assert_eq!(super::count_inversions(vec![1, 3, 5, 2, 4, 6]), 3);
        assert_eq!(super::count_inversions(vec![1, 3, 5, 2, 4, 6]), 3);
        assert_eq!(super::count_inversions(vec![1, 3, 5, 2, 4, 6]), 3);
        assert_eq!(super::count_inversions(vec![1, 3, 5, 2, 4, 6]), 3);
        assert_eq!(super::count_inversions(vec![1, 3, 5, 2, 4, 6]), 3);
        assert_eq!(super::count_inversions(vec![1, 3, 5, 2, 4, 6]), 3);
        assert_eq!(super::count_inversions(vec![1, 3, 5, 2, 4, 6]), 3);
        assert_eq!(super::count_inversions(vec![1, 3, 5, 2, 4, 6]), 3);
    }

    #[test]
    fn test_one_inversion() {
        assert_eq!(super::count_inversions(vec![1, 2]), 0);
        assert_eq!(super::count_inversions(vec![2, 1]), 1);
    }
}

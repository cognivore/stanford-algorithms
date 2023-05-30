trait MergeSortable {
    fn merge_sort(&mut self);
}

impl MergeSortable for Vec<u64> {
    fn merge_sort(&mut self) {
        *self = merge_sort_v64_do(&self[..]);
    }
}

fn merge_sort_v64_do(s_xs: &[u64]) -> Vec<u64> {
    let len = s_xs.len();
    if len < 2 {
        return s_xs.to_vec();
    }
    let mid = len / 2;
    let (left, right) = s_xs.split_at(mid); // <- this returns a slice!
    merge_v64_do(&merge_sort_v64_do(left), &merge_sort_v64_do(right))
}

fn merge_v64_do(left: &[u64], right: &[u64]) -> Vec<u64> {
    let mut l_i = 0;
    let mut r_i = 0;
    let mut result = Vec::with_capacity(left.len() + right.len());
    // Merge the slices, terminating at the shortest slice
    while l_i < left.len() && r_i < right.len() {
        if left[l_i] < right[r_i] {
            result.push(left[l_i]);
            l_i += 1;
        } else {
            result.push(right[r_i]);
            r_i += 1;
        }
    }
    // Now check if there's still some items left in either slice.
    while l_i < left.len() {
        result.push(left[l_i]);
        l_i += 1;
    }
    while r_i < right.len() {
        result.push(right[r_i]);
        r_i += 1;
    }
    result
}

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
        xs.merge_sort();
        // Assert that select(xs) mutates it to match image:
        assert_eq!(xs, image);
    }
}

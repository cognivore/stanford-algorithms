trait MergeSortable {
    fn merge_sort(&mut self);
}

impl MergeSortable for Vec<u64> {
    fn merge_sort(&mut self) {
        merge_sort_v64_do(&mut self[..]);
    }
}

fn merge_sort_v64_do(s_xs: &mut [u64]) {
    let len = s_xs.len();
    if len < 2 {
        return;
    }
    let mid = len / 2;
    merge_sort_v64_do(&mut s_xs[..mid]);
    merge_sort_v64_do(&mut s_xs[mid..]);
    merge_v64_do(s_xs)
}

fn merge_v64_do(s_xs: &mut [u64]) {
    // dbg!("* * * * * * * MERGING * * * * * * *");
    // dbg!(s_xs.to_vec());
    let len = s_xs.len();
    let mid = len / 2;
    let l_len = mid;
    //let r_len = len - mid;
    let mut li = 0;
    let mut ri = mid;
    let mut sli = 0;
    let mut sri = 0;
    let mut ci = 0;

    let mut sl = Vec::new();
    let mut sr = Vec::new();

    /*
    let (l_max, is_l_max_adjusted) = if r_len > l_len {
        (r_len, true)
    } else {
        (l_len, false)
    };
    */
    let l_max = l_len;
    let r_max = len;

    while li < l_max && ri < r_max {
        /* dbg!(
            li,
            l_max,
            ri,
            r_max,
            ci,
            sli,
            sri,
            s_xs.to_vec(),
            sl.clone(),
            sr.clone()
        ); */
        // Subroutine "AccountForLMaxAdjustment(li, is_l_max_adjusted)".
        //if li == l_max - 1 && is_l_max_adjusted {
        //    li = ri; // We account for odd-sized inputs this way.
        //    ri += 1;
        //}

        // Subroutine "StashOverridenItem(x, SL, SR, i, mid)".
        if ci < mid {
            if li <= ci {
                // dbg!("Caching left", s_xs[ci]);
                sl.push(s_xs[ci]);
            }
        } else {
            if ri <= ci {
                // dbg!("Caching right", s_xs[ci]);
                sr.push(s_xs[ci]);
            }
        }
        // dbg!(sl.clone());
        // dbg!(sr.clone());

        // Subroutine "HandleStashedItem(s_xs, SL, SR, sli, sri, li, ri)".
        /* dbg!(
            sl.len() > sli,
            sr.len() > sri,
            ci >= li,
            ci >= ri,
            sl.len() > sli && (sr.len() <= sri || sl[sli] <= sr[ri]),
            sr.len() > sri && (sl.len() <= sli || sr[sri] <= sl[li])
        ); */
        if sl.len() > sli
            && (ci >= ri || sl[sli] <= s_xs[ri])
            && (sr.len() <= sri || (sr.len() > sri && sl[sli] <= sr[sri]))
        {
            // dbg!("Cached left is smaller");
            s_xs[ci] = sl[sli];
            sli += 1;
            li += 1;
        } else if sr.len() > sri
            && (ci >= li || sr[sri] <= s_xs[li])
            && (sl.len() <= sri || (sl.len() > sli && sr[sri] <= sl[sli]))
        {
            // dbg!("Cached right is smaller");
            s_xs[ci] = sr[sri];
            sri += 1;
            ri += 1;
        } else if ci <= li && ci <= ri {
            // Subroutine "Base(s_xs, li, ri, mid)". We can do that because we already know
            // that there is nothing "small" in stashes.
            if s_xs[li] <= s_xs[ri] {
                // dbg!("Left is smaller");
                s_xs[ci] = s_xs[li];
                li += 1;
            } else {
                // dbg!("Right is smaller");
                s_xs[ci] = s_xs[ri];
                ri += 1;
            }
        } else {
            if ci <= li {
                // dbg!("Force read right");
                if s_xs[li] <= sr[sri] {
                    // dbg!("(force right) Left is smaller");
                    s_xs[ci] = s_xs[li];
                    li += 1;
                } else {
                    // dbg!("(force right) Right is smaller");
                    s_xs[ci] = sr[sri];
                    ri += 1;
                    sri += 1;
                }
            } else if ci <= ri {
                // dbg!("Force read left");
                if sl[sli] <= s_xs[ri] {
                    // dbg!("(force left) Left is smaller");
                    s_xs[ci] = sl[sli];
                    li += 1;
                    sli += 1;
                } else {
                    // dbg!("(force left) Right is smaller");
                    s_xs[ci] = s_xs[ri];
                    ri += 1;
                }
            } else {
                // dbg!("Force both");
                if sl[sli] <= sr[sri] {
                    // dbg!("(force both) Left is smaller");
                    s_xs[ci] = sl[sli];
                    li += 1;
                    sli += 1;
                } else {
                    // dbg!("(force both) Right is smaller");
                    s_xs[ci] = sr[sri];
                    ri += 1;
                    sri += 1;
                }
            }
        }
        ci += 1;
    }

    // Subroutine "HandleRemainingItems(s_xs, SL, SR, sli, sri, li, ri)".
    if li < l_max {
        // dbg!("Left is remaining");
        for i in li..l_max {
            if sli < sl.len() {
                s_xs[ci] = sl[sli];
                sli += 1;
            } else {
                s_xs[ci] = s_xs[i];
            }
            ci += 1;
        }
    }
    if ri < r_max {
        // dbg!("Right is remaining");
        for i in ri..r_max {
            if sri < sr.len() {
                s_xs[ci] = sr[sri];
                sri += 1;
            } else {
                s_xs[ci] = s_xs[i];
            }
            ci += 1;
        }
    }

    // dbg!(s_xs.to_vec());
    // dbg!("* * * * * * * MERGED * * * * * * *");
}

#[cfg(test)]
mod tests {
    use rand::seq::SliceRandom;

    use super::*;

    #[test]
    fn simple() {
        let mut xs = vec![1, 0];
        xs.merge_sort();
        assert_eq!(xs, vec![0, 1]);
    }

    #[test]
    fn simple3() {
        let mut xs = vec![1, 2, 0];
        xs.merge_sort();
        assert_eq!(xs, vec![0, 1, 2]);
    }

    #[test]
    fn t2310134() {
        let mut xs = vec![2, 3, 10, 1, 3, 4];
        xs.merge_sort();
        assert_eq!(xs, vec![1, 2, 3, 3, 4, 10]);
    }

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

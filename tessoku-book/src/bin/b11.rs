use std::cmp::Ordering;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        q: usize,
        x: [usize; q]
    }

    a.sort();

    for i in 0..q {
        let result = a
            .binary_search_by(|&xx| {
                if xx >= x[i] {
                    return Ordering::Greater;
                } else {
                    return Ordering::Less;
                }
            })
            .unwrap_or_else(|n| n);

        println!("{:?}", result)
    }
}

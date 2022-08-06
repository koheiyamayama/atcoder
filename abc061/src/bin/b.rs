use std::result;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [(usize, usize); m]
    }

    let mut results = vec![0; n];

    for aa in a {
        results[aa.0 - 1] += 1;
        results[aa.1 - 1] += 1;
    }

    for r in results {
        println!("{}", r)
    }
}

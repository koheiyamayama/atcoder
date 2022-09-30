use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        e: usize
    }

    let vec = vec![a, b, c, d, e];
    let vec: HashSet<usize> = vec.into_iter().collect();
    println!("{}", vec.len())
}

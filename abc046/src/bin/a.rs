use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize
    }

    let penki: HashSet<usize> = vec![a, b, c].into_iter().collect();

    println!("{}", penki.len());
}

use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut l: [usize; n]
    }

    l.sort_by_key(|&m| Reverse(m));

    let mut sum = 0;
    for i in &l[..k] {
        sum += i;
    }
    println!("{}", sum)
}

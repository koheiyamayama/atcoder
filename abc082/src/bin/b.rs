use std::cmp::Reverse;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
        mut t: Chars
    }

    s.sort();
    t.sort_by_key(|&m| Reverse(m));

    if s < t {
        println!("Yes")
    } else {
        println!("No")
    }
}

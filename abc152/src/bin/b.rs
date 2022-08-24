use std::fmt::format;

use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize
    }

    let aa = format!("{}", a).repeat(b);
    let bb = format!("{}", b).repeat(a);

    if aa < bb {
        println!("{}", aa)
    } else {
        println!("{}", bb)
    }
}

use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n]
    }

    let mut set: HashSet<String> = HashSet::new();
    for i in 0..n {
        set.insert(s[i].to_string());
    }
    println!("{}", set.len())
}

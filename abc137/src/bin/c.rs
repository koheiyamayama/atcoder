use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut s: [Chars; n]
    }

    let mut map: HashMap<String, usize> = HashMap::new();
    for i in 0..n {
        s[i].sort();
        let key: String = s[i].clone().into_iter().collect();
        *map.entry(key).or_insert(0) += 1;
    }

    let mut count = 0;
    for (k, v) in map {
        count += v * (v - 1) / 2;
    }

    println!("{}", count)
}

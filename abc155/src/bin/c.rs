use std::{cmp::Reverse, collections::HashMap};

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n]
    }

    let mut map: HashMap<String, usize> = HashMap::new();
    for i in 0..n {
        *map.entry(s[i].to_string()).or_insert(1) += 1;
    }

    let mut dict = map.into_iter().collect_vec();
    dict.sort_by(|x, y| x.0.cmp(&y.0));
    dict.sort_by(|x, y| y.1.cmp(&x.1));

    let max = dict[0].1;
    for i in 0..dict.len() {
        if dict[i].1 == max {
            println!("{}", dict[i].0)
        }
    }
}

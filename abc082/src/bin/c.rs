use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut map: HashMap<usize, usize> = HashMap::new();

    for i in 0..n {
        *map.entry(a[i]).or_insert(0) += 1;
    }

    let mut count = 0;
    for (key, value) in map {
        if key == value {
            count += 0;
        } else if key > value {
            count += value;
        } else {
            count += value - key;
        }
    }

    println!("{}", count)
}

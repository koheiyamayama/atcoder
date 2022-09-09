use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut map: HashMap<usize, usize> = HashMap::new();
    for i in 0..n {
        input! {
            k: usize,
            a: [usize; k]
        }

        for i in 0..k {
            *map.entry(a[i]).or_insert(0) += 1;
        }
    }

    let mut count = 0;
    for (key, value) in map {
        if value == n {
            count += 1;
        }
    }

    println!("{}", count)
}

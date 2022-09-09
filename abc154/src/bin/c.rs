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

    for (key, value) in map {
        if value >= 2 {
            println!("NO");
            return;
        }
    }

    println!("YES")
}

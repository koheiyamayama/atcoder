use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut map: HashMap<usize, usize> = HashMap::new();
    for i in 0..n {
        match map.get_mut(&a[i]) {
            Some(v) => {
                if *v == 0 {
                    *v += 1;
                } else {
                    *v -= 1;
                }
            }
            None => {
                map.insert(a[i], 1);
            }
        }
    }

    let mut count = 0;
    for (key, value) in map {
        if value > 0 {
            count += 1;
        }
    }

    println!("{}", count)
}

use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n]
    }

    let mut map: HashMap<usize, usize> = HashMap::new();
    for i in 0..n {
        *map.entry(a[i]).or_insert(0) += 1;
    }

    let mut dict = map.values().collect::<Vec<_>>();
    dict.sort();
    let len = dict.len();
    if len > k {
        let end = len - k;
        let mut sum = 0;
        for i in dict[0..end].into_iter() {
            sum += **i;
        }
        println!("{}", sum)
    } else {
        println!("0")
    }
}

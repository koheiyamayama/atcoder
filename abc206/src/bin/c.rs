use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut map: HashMap<usize, usize> = HashMap::new();
    let mut count = 0;
    for i in 0..n {
        let v = map.get(&a[i]);
        if v != None {
            count += i - v.unwrap();
        } else {
            count += i;
        }
        *map.entry(a[i]).or_insert(0) += 1;
    }

    println!("{}", count)
}

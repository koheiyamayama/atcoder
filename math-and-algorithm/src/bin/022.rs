// use proconio::input;

// fn main() {
//     input! {
//         n: usize,
//         a: [usize; n]
//     }

//     let mut count = 0;
//     for i in 0..n {
//         for m in i + 1..n {
//             let sum = a[i] + a[m];
//             // println!("{}, {}", a[i], a[m]);

//             if sum == 100_000 {
//                 count += 1;
//             }
//         }
//     }

//     println!("{}", count)
// }

use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut count = 0;
    let mut map: HashMap<usize, usize> = HashMap::new();

    for i in a {
        *map.entry(i).or_insert(0) += 1;
    }

    for (&k, &v) in map.iter() {
        if 50_001 <= k {
            continue;
        }

        let s = 100_000 - k;
        let r = match map.get(&s) {
            Some(vv) => *vv,
            None => { continue; }
        };

        if k == 50_000 {
            count = count + combination_two(r as u64) as usize;
        } else {
            count = count + (r * v);
        }
    }

    println!("{}", count)
}

fn combination_two(n: u64) -> u64 { 
    return n * (n - 1) / 2
 }

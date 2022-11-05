use std::cmp::Ordering;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        d: usize,
        lr: [(Usize1, usize); n]
    }

    let mut used = vec![false; n];
    let mut cum = vec![0; d];
    let mut sum = 0;
    for i in 0..d {
        let mut max = 0;
        let mut task_id = -1;
        for j in 0..n {
            if !used[j] && lr[j].0 <= i {
                // max = lr[j].1.max(max);
                max = if lr[j].1 >= max {
                    task_id = j as i32;
                    lr[j].1
                } else {
                    max
                }
            }
        }
        if task_id != -1 {
            used[task_id as usize] = true;
        }
        sum += max;
    }

    println!("{:?}", sum);
}

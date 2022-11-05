use proconio::{input, marker::Usize1};

fn main() {
    input! {
        d: usize,
        n: usize,
        lrh: [(usize, usize, usize); n]
    }

    let mut counter = vec![24; d + 1];
    counter[0] = 0;
    for i in 0..n {
        for j in lrh[i].0..=lrh[i].1 {
            counter[j] = counter[j].min(lrh[i].2);
        }
    }

    let mut sum = 0;
    for i in 0..d + 1 {
        sum += counter[i];
    }

    println!("{}", sum)
}

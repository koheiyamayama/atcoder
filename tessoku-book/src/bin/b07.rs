use proconio::{input, marker::Usize1};

fn main() {
    input! {
        t: usize,
        n: usize,
        lr: [(usize, usize); n]
    }

    let mut a = vec![0; t + 1];
    for i in 0..n {
        a[lr[i].0] += 1;
        a[lr[i].1] -= 1;
    }

    let mut acum = vec![0; t];
    acum[0] = a[0];
    for i in 1..t {
        acum[i] = acum[i - 1] + a[i];
    }

    for i in 0..acum.len() {
        println!("{}", acum[i])
    }
}

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        d: usize,
        n: usize,
        lr: [(Usize1, Usize1); n]
    }

    let mut a = vec![0; d + 1];
    for i in 0..n {
        a[lr[i].0] += 1;
        a[lr[i].1 + 1] -= 1;
    }

    let mut acum = vec![0; d];
    acum[0] = a[0];
    for i in 1..d {
        acum[i] = a[i] + acum[i - 1];
    }

    for i in 0..acum.len() {
        println!("{}", acum[i])
    }
}

use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        lr: [(usize, usize); q]
    }

    let mut acum: Vec<usize> = vec![0; n + 1];
    for i in 1..n + 1 {
        acum[i] = a[i - 1] + acum[i - 1];
    }

    println!("{:?}", acum);

    for (l, r) in lr.iter() {
        println!("{}", acum[*r as usize] - acum[l - 1])
    }
}

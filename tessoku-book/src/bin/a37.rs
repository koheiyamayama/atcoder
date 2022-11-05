use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        b: usize,
        a: [usize; n],
        c: [usize; m]
    }

    let mut d = 0;
    for i in 0..n {
        d += a[i] + b;
    }

    let mut e = 0;
    for i in 0..m {
        e += c[i];
    }

    println!("{}", d * m + e * n)
}

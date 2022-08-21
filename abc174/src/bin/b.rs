use proconio::input;

fn main() {
    input! {
        n: usize,
        d: isize,
        a: [(isize, isize); n]
    }

    let mut count = 0;
    for i in 0..n {
        if distance(a[i].0, a[i].1) <= d as f64 {
            count += 1;
        }
    }

    println!("{}", count)
}

fn distance(a: isize, b: isize) -> f64 {
    return ((a * a + b * b) as f64).sqrt();
}

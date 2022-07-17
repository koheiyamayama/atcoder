use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n]
    }

    let mut sum = 0.0;
    for i in 0..n {
        sum += (a[i] as f64) * 0.333333333333333333333333 + (b[i] as f64) * 0.6666666666666666666666666666666;
    }

    println!("{:.12}", sum)
}

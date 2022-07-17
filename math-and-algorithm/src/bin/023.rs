use proconio::input;

fn main() {
    input! {
        n: usize,
        b: [usize; n],
        r: [usize; n]
    }

    let mut total_b: f64 = 0.0;
    for i in b {
        total_b += i as f64;
    }

    let mut total_r: f64 = 0.0;
    for i in r {
        total_r += i as f64;
    }

    let n = n as f64;

    println!("{:.12}", total_b / n + total_r / n)

}

use proconio::input;

fn main() {
    input! {
        n: f64
    }

    let mut l = 0.0;
    let mut r = 100.0;
    let mut m = 0.0;

    for i in 0..20 {
        m = (l + r) / 2.0;
        if m * m * m + m > n {
            r = m;
        } else {
            l = m;
        }
    }

    println!("{:.6}", m)
}

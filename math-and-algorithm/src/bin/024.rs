use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [(usize, usize); n]
    }

    let mut total = 0.0;
    for (b, c) in a {
        total += c as f64 / b as f64;
    }

    println!("{:.12}", total)
}

use proconio::input;

fn main() {
    input! {
        n: i64,
        m: i64
    }

    let r: i64;
    if n * 2 <= m {
        r = n + (m - (n * 2)) / 4;
    } else {
        r = m / 2
    }

    println!("{}", r)
}

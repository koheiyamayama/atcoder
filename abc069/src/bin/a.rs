use proconio::input;

fn main() {
    input! {
        m: usize,
        n: usize
    }

    println!("{}", (m - 1) * (n - 1))
}

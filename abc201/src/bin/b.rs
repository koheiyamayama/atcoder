use proconio::input;

fn main() {
    input! {
        n: usize,
        mut s: [(String, usize); n]
    }

    s.sort_by(|a, b| a.1.cmp(&b.1));
    println!("{}", s[n - 2].0)
}

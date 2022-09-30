use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars
    }

    let mut count = 1;
    for i in 1..n {
        let prev = s[i - 1];
        if prev != s[i] {
            count += 1;
        }
    }

    println!("{}", count)
}

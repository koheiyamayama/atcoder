use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }

    let first = s[0];
    let last = s.last().unwrap();
    let len = s.len();

    let mut count = 0;
    for _ in 1..len - 1 {
        count += 1;
    }

    println!("{}{}{}", first, count, last)
}

use proconio::input;
use proconio::marker::{Chars};

fn main() {
    input! {
        s: Chars
    }

    let mut results: Vec<String> = Vec::new();

    for c in s {
        if c == '0' {
            results.push(c.to_string());
        } else if c == '1' {
            results.push(c.to_string())
        } else if c == 'B' {
            results.pop();
        }
    }

    println!("{}", results.join(""))
}

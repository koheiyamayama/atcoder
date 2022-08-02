use proconio::{input, marker::Chars};

fn main() {
    input! {
        a: Chars,
        b: Chars,
        c: Chars
    }

    if a.last().unwrap() == b.first().unwrap() && b.last().unwrap() == c.first().unwrap() {
        println!("YES")
    } else {
        println!("NO")
    }
}

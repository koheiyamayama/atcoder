use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        a: String,
        b: Chars,
        c: String,
    }

    println!("A{}C", b[0])
}

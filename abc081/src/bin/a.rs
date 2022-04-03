use proconio::input;
use proconio::marker::{Chars};

fn main() {
    input! {
        a: Chars
    }

    let num: u32 = a.iter().map(|b| {b.to_digit(10).unwrap()}).sum();

    println!("{}", num)
}

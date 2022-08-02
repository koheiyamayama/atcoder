use proconio::{input, marker::Chars};

fn main() {
    input! {
        a: Chars,
        b: Chars,
        c: Chars
    }

    let letters = vec![a[0].to_string(), b[0].to_string(), c[0].to_string()];
    println!("{}", letters.join("").to_uppercase())
}

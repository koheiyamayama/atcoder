use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: u8,
        s: Chars
    }

    for i in 0..s.len() {
        let u = s[i as usize] as u8 + n;
        if u > 90 {
            print!("{}", (n - (90 - s[i] as u8) + 64) as char)
        } else {
            print!("{}", u as char)
        }
    }

    println!("");
}

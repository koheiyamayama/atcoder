use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut s: Chars
    }

    s[5] = ' ';
    s[13] = ' ';

    let s: String = s.iter().collect();

    println!("{}", s);
}

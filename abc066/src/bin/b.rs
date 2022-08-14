use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }
    let len = s.len();

    for i in 0..len {
        let s = &s[0..len - i];
        let half = s.len() / 2;
        let first = &s[0..half];
        let last = &s[half..];

        if i != 0 && first == last {
            println!("{}", first.len() + last.len());
            break;
        }
    }
}

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }

    let before = &s[0..s.len() / 2];
    let after = if s.len() % 2 == 0 {
        &s[s.len() / 2..]
    } else {
        &s[s.len() / 2 + 1..]
    };

    let mut count = 0;
    for i in 0..s.len() / 2 {
        if before[i] == after[s.len() / 2 - 1 - i] {
        } else {
            count += 1;
        }
    }

    println!("{}", count)
}

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars
    }

    let mut count = std::usize::MAX;
    for i in 0..=s.len() - t.len() {
        let mut diff = 0;
        let ss = &s[i..(i + t.len())];
        for j in 0..t.len() {
            if ss[j] != t[j] {
                diff += 1;
            }
        }
        count = diff.min(count);
    }

    println!("{}", count)
}

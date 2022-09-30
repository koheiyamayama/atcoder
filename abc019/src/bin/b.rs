use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }

    let mut ss: Vec<(char, usize)> = Vec::new();
    ss.push((s[0], 1));

    for i in 1..s.len() {
        if ss.last().unwrap().0 == s[i] {
            ss.last_mut().unwrap().1 += 1;
        } else {
            ss.push((s[i], 1))
        }
    }

    for i in 0..ss.len() {
        print!("{}{}", ss[i].0, ss[i].1)
    }
    println!("");
}

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        k: usize
    }

    let mut kukan: Vec<(char, usize)> = vec![(s[0], 1)];

    for i in 1..s.len() {
        let last = *kukan.last().unwrap();

        if last.0 == s[i] {
            kukan.last_mut().unwrap().1 += 1;
        } else {
            kukan.push((s[i], 1))
        }
    }

    let mut sum = 0;
    for i in 0..kukan.len() {
        sum += kukan[i].1 / 2
    }
    sum *= k;

    let first = *kukan.first().unwrap();
    let last = *kukan.last().unwrap();
    if first.0 == last.0 && first.1 % 2 != 0 && last.1 % 2 != 0 {
        sum += k - 1
    }

    println!("{}", sum)
}

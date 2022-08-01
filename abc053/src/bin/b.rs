use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }

    let mut a_index = std::usize::MAX;
    let mut z_index = std::usize::MIN;
    let mut index = 0;
    for c in s.clone() {
        if c == 'A' {
            a_index = min(a_index, index)
        } else if c == 'Z' {
            z_index = max(z_index, index)
        }
        index += 1;
    }

    println!("{}", &s[a_index..=z_index].len())
}

fn max(n: usize, m: usize) -> usize {
    if n >= m {
        return n;
    } else {
        return m;
    }
}

fn min(n: usize, m: usize) -> usize {
    if n <= m {
        return n;
    } else {
        return m;
    }
}

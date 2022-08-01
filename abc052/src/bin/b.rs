use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: isize,
        s: Chars
    }

    let mut count = 0;
    let mut result = 0;
    for c in s {
        if c == 'I' {
            count += 1;
        } else {
            count -= 1;
        }

        result = max(result, count)
    }

    println!("{}", result)
}

fn max(n: isize, m: isize) -> isize {
    if n >= m {
        return n;
    } else {
        return m;
    }
}

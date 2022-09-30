use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }
    let mut a: Vec<usize> = vec![0; s.len() + 1];

    for i in 0..s.len() {
        if s[i] == '<' {
            a[i + 1] = a[i].max(a[i] + 1)
        }
    }

    for i in (0..s.len()).rev() {
        if s[i] == '>' {
            a[i] = a[i].max(a[i + 1] + 1)
        }
    }

    println!("{:?}", a.iter().sum::<usize>())
}

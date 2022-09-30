use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars
    }

    let mut index = 1;
    let mut sum = 0;

    for i in (0..n.len()).rev() {
        sum += index * n[i].to_digit(10).unwrap() as usize;
        index *= 2;
    }

    println!("{}", sum)
}

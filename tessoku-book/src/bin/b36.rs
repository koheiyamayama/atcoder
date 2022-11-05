use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars
    }

    let mut count = 0;
    for i in 0..n {
        if s[i] == '1' {
            count += 1;
        }
    }

    if count % 2 == 0 && k % 2 == 0 {
        println!("Yes")
    } else if count % 2 != 0 && k % 2 != 0 {
        println!("Yes")
    } else {
        println!("No")
    }
}

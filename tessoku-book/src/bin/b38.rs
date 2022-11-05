use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars
    }

    let mut ans = vec![1; n];
    for i in 1..n {
        if s[i - 1] == 'A' {
            ans[i] = ans[i - 1] + 1;
        }
    }

    for i in (1..n).rev() {
        if s[i - 1] == 'B' {
            ans[i - 1] = if ans[i - 1] > ans[i] {
                ans[i - 1]
            } else {
                ans[i] + 1
            }
        }
    }

    let mut sum = 0;
    for i in 0..n {
        sum += ans[i];
    }
    println!("{}", sum)
}

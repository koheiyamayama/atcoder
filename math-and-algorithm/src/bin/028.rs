use proconio::input;
use std::vec::Vec;

fn main() {
    input! {
        n: usize,
        h: [usize; n]
    }

    let mut dp: Vec<usize> = vec![0; n];

    for i in 0..n {
        if i == 0 {
            dp[i] = 0;
        }

        if i == 1 {
            dp[i] = abs_diff(h[i], h[i-1]);
        }

        if 2 <= i {
            let one = dp[i - 1] + abs_diff(h[i], h[i-1]);
            let two = dp[i - 2] + abs_diff(h[i], h[i-2]);

            dp[i] = min(one, two);
        }
    }

    println!("{}", dp[n-1])
}

fn abs_diff(n: usize, m: usize) -> usize {
    if n <= m {
        return m - n
    } else {
        return n - m
    }
}

fn min(n: usize, m: usize) -> usize {
    if n <= m {
        return n
    } else {
        return m
    }
}

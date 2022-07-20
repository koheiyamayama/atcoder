use std::vec;

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        c: [usize; n]
    }

    let mut dp = vec![vec![false; s+1]; n+1];

    for i in 0..=n {
        for m in 0..=s {
            if i == 0 {
                if m == 0 {
                    dp[i][m] = true;
                    break;
                }
            }

            if m < c[i-1] {
                dp[i][m] = dp[i-1][m];
            }

            if m >= c[i-1] {
                if dp[i-1][m] == true || dp[i-1][m-c[i-1]] {
                    dp[i][m] = true
                } else {
                    dp[i][m] = false
                }
            }
        }
    }

    if dp[n][s] {
        println!("Yes")
    } else {
        println!("No")
    }
}

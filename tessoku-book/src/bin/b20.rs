use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars
    }

    let mut dp = vec![vec![0; s.len() + 1]; t.len() + 1];
    dp[0][0] = 0;

    for i in 0..t.len() + 1 {
        for j in 0..s.len() + 1 {
            if i == 0 && j == 0 {
                continue;
            }

            let mut l: usize = 3_000;
            if j >= 1 {
                l = dp[i][j - 1] + 1;
            }
            let mut u: usize = 3_000;
            if i >= 1 {
                u = dp[i - 1][j] + 1;
            };
            let n = if j >= 1 && i >= 1 && s[j - 1] == t[i - 1] {
                dp[i - 1][j - 1]
            } else if j >= 1 && i >= 1 {
                dp[i - 1][j - 1] + 1
            } else {
                3_000
            };
            dp[i][j] = l.min(u).min(n)
        }
    }

    println!("{}", dp[t.len()][s.len()])
}

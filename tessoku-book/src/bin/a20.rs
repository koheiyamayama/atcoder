use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars
    }

    let mut dp = vec![vec![0; s.len() + 1]; t.len() + 1];

    for i in 0..t.len() + 1 {
        for j in 0..s.len() + 1 {
            if j >= 1 {
                dp[i][j] = dp[i][j - 1]
            }

            if i >= 1 {
                dp[i][j] = dp[i - 1][j].max(dp[i][j])
            }

            if i >= 1 && j >= 1 && t[i - 1] == s[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            }
        }
    }

    println!("{}", dp[t.len()][s.len()])
}

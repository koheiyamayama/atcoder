use proconio::input;

fn main() {
    input! {
        n: usize,
        w: usize,
        wv: [(usize, usize); n]
    }

    let max_value = 100_000;
    let mut dp = vec![vec![std::usize::MAX; max_value + 1]; n + 1];
    dp[0][0] = 0;
    for i in 1..n + 1 {
        for j in 0..max_value + 1 {
            dp[i][wv[i - 1].1] = wv[i - 1].0;
            dp[i][j] = dp[i - 1][j].min(dp[i][j]);
            if j >= wv[i - 1].1 && dp[i - 1][j - wv[i - 1].1] != std::usize::MAX {
                dp[i][j] = dp[i][j].min(dp[i - 1][j - wv[i - 1].1] + wv[i - 1].0);
            }
        }
    }

    let mut max = 0;
    for i in 0..n + 1 {
        for j in 0..max_value + 1 {
            if dp[i][j] <= w {
                max = max.max(j)
            }
        }
    }

    println!("{}", max)
}

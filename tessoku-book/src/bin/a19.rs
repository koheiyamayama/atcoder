use proconio::input;

fn main() {
    input! {
        n: usize,
        w: usize,
        wv: [(usize, usize); n]
    }

    let mut dp = vec![vec![0; w + 1]; n + 1];
    for i in 1..n + 1 {
        for j in 0..w + 1 {
            if j == wv[i - 1].0 {
                dp[i][j] = wv[i - 1].1;
            }

            let r: usize = dp[i - 1][j];
            let b: usize;
            if j >= wv[i - 1].0 {
                b = dp[i - 1][j - wv[i - 1].0] + wv[i - 1].1;
                dp[i][j] = r.max(b);
            } else {
                dp[i][j] = r.max(dp[i][j]);
            }
        }
    }

    println!("{}", dp[n][w])
}

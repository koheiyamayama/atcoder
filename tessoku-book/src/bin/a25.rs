use proconio::{input, marker::Chars};

fn main() {
    input! {
      h: usize,
      w: usize,
      c: [Chars; h]
    }

    let mut dp: Vec<Vec<u128>> = vec![vec![0; w + 1]; h + 1];
    dp[1][1] = 1;
    for i in 1..h + 1 {
        for j in 1..w + 1 {
            if i == 1 && j == 1 {
                continue;
            }

            dp[i][j] = if c[i - 1][j - 1] == '.' {
                dp[i - 1][j] + dp[i][j - 1]
            } else {
                0
            }
        }
    }

    println!("{}", dp[h][w])
}

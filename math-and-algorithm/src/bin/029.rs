use proconio::input;

fn main() {
    input! {
        n: usize
    }

    let mut dp = vec![0; n];

    for i in 0..n {
        if i == 0 {
            dp[i] = 1;
        }

        if i == 1 {
            dp[i] = dp[i-1] + 1;
        }

        if 2 <= i {
            dp[i] = dp[i-1] + dp[i-2];
        }
    }

    println!("{}", dp[n-1])
}

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize;n-1],
        b: [usize;n-1]
    }

    let mut dp = vec![std::isize::MIN; n];
    dp[0] = 0;

    for i in 0..n - 1 {
        dp[a[i] - 1] = (dp[i] + 100).max(dp[a[i] - 1]);
        dp[b[i] - 1] = (dp[i] + 150).max(dp[b[i] - 1]);
    }

    println!("{:?}", dp[n - 1])
}

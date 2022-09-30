use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n-1],
        b: [usize; n-2]
    }

    let mut dp = vec![std::usize::MAX; n];
    for i in 0..n {
        if i == 0 {
            dp[i] = 0;
            continue;
        }

        if i == 1 {
            dp[i] = a[0];
            continue;
        }

        dp[i] = (dp[i - 1] + a[i - 1]).min(dp[i - 2] + b[i - 2]);
    }

    println!("{}", dp.last().unwrap())
}

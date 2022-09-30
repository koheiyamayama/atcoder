use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [usize; n]
    }

    let mut dp = vec![0; n];
    for i in 0..n {
        if i == 0 {
            dp[i] = 0;
            continue;
        }

        if i == 1 {
            dp[i] = abs_diff(h[i], h[i - 1]);
            continue;
        }

        dp[i] = (dp[i - 2] + abs_diff(h[i], h[i - 2])).min(dp[i - 1] + abs_diff(h[i], h[i - 1]));
    }

    println!("{}", dp.last().unwrap())
}

fn abs_diff(n: usize, m: usize) -> usize {
    if n <= m {
        return m - n;
    } else {
        return n - m;
    }
}

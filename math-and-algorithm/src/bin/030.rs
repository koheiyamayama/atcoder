use proconio::input;

fn main() {
    input! {
        n: usize,
        w: usize,
        items: [(usize, usize); n]
    }

    let mut dp: Vec<Vec<usize>> = vec![vec![0; w+1]; n+1];

    for i in 0..=n {
        for j in 0..=w {
            if i < 1 {
                dp[i][j] = 0;
                continue;
            }

            if items[i-1].0 > j {
                dp[i][j] = dp[i-1][j];
            }
            if items[i-1].0 <= j {
                dp[i][j] = max(dp[i-1][j], dp[i-1][j-items[i-1].0] + items[i-1].1);
            }
        }
    }

    let mut ans = 0;

    for a in dp {
        let max = *a.iter().max().unwrap();

        if max > ans {
            ans = max;
        }
    }

    println!("{}", ans)

}

fn max(a: usize, b: usize) -> usize {
    if a <= b {
        return b
    } else {
        return a
    }
}

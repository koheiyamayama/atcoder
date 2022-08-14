use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [isize; n]
    }

    let mut dp = vec![0; n - 1];
    let mut dp1 = vec![0; n - 1];
    let total: isize = a.iter().sum();

    for i in 0..n - 1 {
        if i == 0 {
            dp[i] = abs_diff(a[i], total - a[i]);
            dp1[i] = a[i];
            continue;
        }
        let x = dp1[i - 1] + a[i];
        let y = total - x;
        dp[i] = abs_diff(x, y);
        dp1[i] = x;
    }

    // println!("dp: {:?}, dp1: {:?}", dp, dp1)
    println!("{}", dp.iter().min().unwrap())
}

// isizeの絶対値計算
fn abs_diff(mut n: isize, mut m: isize) -> isize {
    if n >= 0 && m >= 0 {
        if n <= m {
            return m - n;
        } else {
            return n - m;
        }
    } else if n >= 0 && m < 0 {
        return n + -1 * m;
    } else if n < 0 && m >= 0 {
        return -1 * n + m;
    } else if n < 0 && m < 0 {
        n = n * -1;
        m = m * -1;
        if n <= m {
            return m - n;
        } else {
            return n - m;
        }
    }
    panic!("絶対値が計算できなかった")
}

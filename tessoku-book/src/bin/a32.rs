use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize
    }

    let mut dp = vec![false; n];

    for i in 2..n as usize {
        if i >= a && dp[i - a] == false {
            dp[i] = true;
        } else if i >= b && dp[i - b] == false {
            dp[i] = false
        } else {
            dp[i] = false;
        }
    }

    if dp[n - 1] {
        println!("First")
    } else {
        println!("Second")
    }
}

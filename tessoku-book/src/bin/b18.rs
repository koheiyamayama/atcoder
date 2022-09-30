use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        a: [usize; n]
    }

    let mut dp = vec![vec![false; s + 1]; n + 1];
    dp[0][0] = true;

    for i in 1..n + 1 {
        for j in 0..s + 1 {
            if dp[i - 1][j] {
                dp[i][j] = true;
                continue;
            }

            if j >= a[i - 1] {
                dp[i][j] = dp[i - 1][j - a[i - 1]]
            }
        }
    }

    if !dp[n][s] {
        println!("-1");
        return;
    }

    let mut routes: Vec<usize> = Vec::new();
    let mut place = s;
    for i in (1..n + 1).rev() {
        if dp[i - 1][place] {
            place = place - 0;
        } else {
            place = place - a[i - 1];
            routes.push(i);
        }
    }

    routes.reverse();

    println!("{}", routes.len());
    for i in 0..routes.len() {
        if i == routes.len() - 1 {
            print!("{}", routes[i]);
        } else {
            print!("{} ", routes[i]);
        }
    }
    println!("")
}

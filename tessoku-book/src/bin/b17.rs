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

    let mut routes: Vec<usize> = Vec::new();
    let mut place: usize = n - 1;
    loop {
        routes.push(place);
        if place == 0 {
            break;
        }

        if dp[place] == abs_diff(h[place], h[place - 1]) + dp[place - 1] {
            place -= 1;
        } else {
            place -= 2;
        }
    }

    routes.reverse();

    println!("{}", routes.len());
    for i in 0..routes.len() {
        if i == routes.len() - 1 {
            print!("{}", routes[i] + 1)
        } else {
            print!("{} ", routes[i] + 1)
        }
    }
    println!("")
}

fn abs_diff(n: usize, m: usize) -> usize {
    if n <= m {
        return m - n;
    } else {
        return n - m;
    }
}

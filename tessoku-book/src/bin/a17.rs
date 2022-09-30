use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n-1],
        b: [usize; n-2]
    }

    let mut dp = vec![std::usize::MAX; n];
    let mut routes: Vec<usize> = Vec::new();

    for i in 0..n {
        if i == 0 {
            dp[i] = 0;
            continue;
        }

        if i == 1 {
            dp[i] = a[i - 1];
            continue;
        }

        dp[i] = (dp[i - 1] + a[i - 1]).min(dp[i - 2] + b[i - 2]);
    }

    let mut place = n - 1;
    while true {
        routes.push(place);
        if place == 0 {
            break;
        }

        if place == 1 {
            routes.push(0);
            break;
        }

        if dp[place - 1] + a[place - 1] == dp[place] {
            place = place - 1;
        } else {
            place = place - 2;
        }
    }

    routes.reverse();

    println!("{}", routes.len());
    for i in 0..routes.len() {
        if i == routes.len() - 1 {
            print!("{}", routes[i] + 1);
        } else {
            print!("{} ", routes[i] + 1);
        }
    }
    println!("")
}

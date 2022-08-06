use proconio::input;

fn main() {
    input! {
        n: u128,
        m: u128
    }

    let mod007 = 1000000007;
    if n == m {
        let mut nn = 1;
        let mut mm = 1;
        for i in 1..=n {
            nn *= i;
            nn %= mod007;
            mm *= i;
            mm %= mod007;
        }

        println!("{}", nn * mm * 2 % mod007);
    } else if abs_diff(n, m) <= 1 {
        let mut nn = 1;
        let mut mm = 1;
        for i in 1..=n {
            nn *= i;
            nn %= mod007;
        }
        for i in 1..=m {
            mm *= i;
            mm %= mod007;
        }

        println!("{}", nn * mm % mod007);
    } else {
        println!("{}", 0)
    }
    // let P = 10u128.pow(9) + 7;

    // let mut ans = 1;
    // for i in 1..=n {
    //     ans *= i;
    //     ans %= P;
    // }
    // println!("{}", ans)
}

fn abs_diff(n: u128, m: u128) -> u128 {
    if n <= m {
        return m - n;
    } else {
        return n - m;
    }
}

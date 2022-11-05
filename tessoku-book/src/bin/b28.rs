use proconio::input;

fn main() {
    input! {
        n: usize
    }

    let mut ans = 1;
    let mut prev = 1;
    for _ in 2..n {
        ans += prev;
        prev = ans - prev;

        ans %= 1000000007;
    }

    println!("{}", ans)
}

use proconio::input;

fn main() {
    input! {
        n: isize,
        k: isize
    }

    let mut count = 0;
    for a in 1..=n {
        for b in 1..=n {
            let c = k - (a + b);
            if 1 <= c && c <= n {
                count += 1;
            }
        }
    }

    println!("{}", count)
}

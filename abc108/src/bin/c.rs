use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize
    }

    for a in 1..=n {
        for b in 1..=n {
            for c in 1..=n {
                if (a + b) % k == 0 && (a + c) % k == 0 && (b + c) % k == 0 {
                    println!("({}, {}, {})", a, b, c)
                }
            }
        }
    }
}

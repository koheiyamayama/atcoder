use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut h: [usize; n]
    }

    h.sort();

    let mut diff = std::usize::MAX;
    for i in 0..=n - k {
        let mi = h[i];
        let max = h[i + k - 1];

        diff = min(diff, max - mi);
    }

    println!("{}", diff)
}

fn min(n: usize, m: usize) -> usize {
    if n <= m {
        return n;
    } else {
        return m;
    }
}

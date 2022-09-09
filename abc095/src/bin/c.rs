use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        ab: usize,
        x: isize,
        y: isize
    }

    let ab_set = ab * 2;

    let mut ans = std::usize::MAX;
    for i in 0..=100_000 {
        let tmp: usize =
            ab_set * i + a * 0.max(x - i as isize) as usize + b * 0.max(y - i as isize) as usize;
        ans = ans.min(tmp)
    }

    println!("{}", ans)
}

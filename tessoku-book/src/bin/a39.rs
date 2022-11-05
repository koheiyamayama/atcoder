use proconio::input;

fn main() {
    input! {
        n: usize,
        mut lr: [(usize, usize); n]
    }

    lr.sort_by(|a, b| a.1.cmp(&b.1));

    let mut ans = 0;
    let mut current_time = 0;
    for i in 0..n {
        if current_time <= lr[i].0 {
            ans += 1;
            current_time = lr[i].1
        }
    }

    println!("{}", ans)
}

use proconio::input;

fn main() {
    input! {
        n: usize,
        d: [usize; n]
    }

    let mut ans = vec![false; 100];
    for i in 0..n {
        ans[d[i] - 1] = true;
    }

    let mut count = 0;
    for i in 0..100 {
        if ans[i] {
            count += 1;
        }
    }

    println!("{}", count)
}

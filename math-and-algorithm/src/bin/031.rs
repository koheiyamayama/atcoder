use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut dp1: Vec<usize> = vec![0; n+1];
    let mut dp2: Vec<usize> = vec![0; n+1];

    for i in 1..=n {
        dp1[i] = a[i-1] + dp2[i-1];
        dp2[i] = max(dp2[i-1], dp1[i-1]);
    }

    if dp1[n] <= dp2[n] {
        println!("{}", dp2[n])
    } else {
        println!("{}", dp1[n])
    }
}

fn max(a: usize, b: usize) -> usize {
    if a <= b {
        return b
    } else {
        return a
    }
}

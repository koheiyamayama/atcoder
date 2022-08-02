use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        tt: [usize; n]
    }

    let mut r = 0;

    // for i in 1..n {
    //     if tt[i] - tt[i - 1] >= t {
    //         r = r + t;
    //     } else {
    //         r = tt[i] - tt[i - 1] + r;
    //     }
    // }

    // if tt.len() == 1 {
    //     println!("{}", t)
    // } else {
    //     println!("{}", r + t)
    // }

    for i in 1..n {
        r += min(tt[i] - tt[i - 1], t);
    }

    println!("{}", r + t)
}

fn min(n: usize, m: usize) -> usize {
    if n <= m {
        return n;
    } else {
        return m;
    }
}

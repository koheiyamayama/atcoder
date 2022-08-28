use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        x: [[isize; d]; n]
    }

    let mut count = 0;
    for i in 0..n {
        for j in i + 1..n {
            let mut acum = 0;
            for m in 0..d {
                acum += abs_diff(x[i][m], x[j][m]) * abs_diff(x[i][m], x[j][m]);
            }
            if is_integer((acum as f64).sqrt()) {
                count += 1;
            }
        }
    }

    println!("{}", count)
}

// isizeの絶対値計算
fn abs_diff(mut n: isize, mut m: isize) -> isize {
    if n >= 0 && m >= 0 {
        if n <= m {
            return m - n;
        } else {
            return n - m;
        }
    } else if n >= 0 && m < 0 {
        return n + -1 * m;
    } else if n < 0 && m >= 0 {
        return -1 * n + m;
    } else if n < 0 && m < 0 {
        n = n * -1;
        m = m * -1;
        if n <= m {
            return m - n;
        } else {
            return n - m;
        }
    }
    panic!("絶対値が計算できなかった")
}

fn is_integer(n: f64) -> bool {
    return n - n.trunc() == 0.0;
}

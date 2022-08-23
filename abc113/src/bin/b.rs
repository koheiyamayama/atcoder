use proconio::input;

fn main() {
    input! {
        n: usize,
        t: isize,
        a: isize,
        h: [isize; n]
    }

    let mut min = std::f64::MAX;
    let mut index = 0;

    for i in 0..n {
        let avg = t as f64 - h[i] as f64 * 0.006;
        let min_avg = t as f64 - min as f64 * 0.006;

        if abs_diff(avg, a as f64) <= abs_diff(min_avg, a as f64) {
            min = h[i] as f64;
            index = i + 1;
        }
    }

    println!("{}", index)
}

// f64の絶対値計算
fn abs_diff(mut n: f64, mut m: f64) -> f64 {
    if n >= 0.0 && m >= 0.0 {
        if n <= m {
            return m - n;
        } else {
            return n - m;
        }
    } else if n >= 0.0 && m < 0.0 {
        return n + -1.0 * m;
    } else if n < 0.0 && m >= 0.0 {
        return -1.0 * n + m;
    } else if n < 0.0 && m < 0.0 {
        n = n * -1.0;
        m = m * -1.0;
        if n <= m {
            return m - n;
        } else {
            return n - m;
        }
    }
    panic!("絶対値が計算できなかった")
}

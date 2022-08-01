use proconio::input;

fn main() {
    input! {
        w: isize,
        a: isize,
        b: isize
    }

    if a + w < b {
        // println!("{}, {}", a + w, b);
        println!("{}", abs_diff(a + w, b));
    } else if a > b + w {
        // println!("{}, {}", b + w, a);
        println!("{}", abs_diff(b + w, a));
    } else {
        println!("0");
    }
}

fn abs_diff(n: isize, m: isize) -> isize {
    if n <= m {
        return m - n;
    } else {
        return n - m;
    }
}

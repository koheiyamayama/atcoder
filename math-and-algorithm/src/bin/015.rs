use proconio::input;

fn main() {
    input! {
        mut a: isize,
        mut b: isize
    }

    let mut is_zero = false;
    while !is_zero {
        if a <= b {
            b = b % a;
        } else {
            a = a % b;
        }

        if a == 0 || b == 0 {
            is_zero = true
        }
    }

    if a <= b {
        print!("{}", b)
    } else {
        print!("{}", a)
    }
}

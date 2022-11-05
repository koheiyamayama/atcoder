use proconio::input;

fn main() {
    input! {
        mut a: usize,
        mut b: usize
    }

    println!("{}", gcd(a, b))
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    let mut ans = 0;
    loop {
        if a <= b {
            b = b % a;
        } else {
            a = a % b;
        }

        if a == 0 {
            ans = b;
            break;
        } else if b == 0 {
            ans = a;
            break;
        }
    }

    return ans;
}

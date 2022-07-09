use proconio::input;

fn main() {
    input! {
        mut a: usize,
        mut b: usize
    }

    println!("{}", gcd(a, b))
}

fn gcd(a: usize, b: usize) -> usize {
    let mut r: usize = 0;

    if a == 0 {
        return b;
    } else if b == 0 {
        return a;
    }    

    if a <= b {
        r = b % a;
        return gcd(a, r)
    } else {
        r = a % b;
        return gcd(r, b)
    }
}

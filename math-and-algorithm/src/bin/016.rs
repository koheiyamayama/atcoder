use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut r: usize = gcd(a[0], a[1]);
    for i in 2..n {
        r = gcd(r, a[i]);
    }

    println!("{}", r);
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


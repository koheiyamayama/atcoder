use proconio::input;

fn main() {
    input! {
        n: u128,
        a: [u128; n]
    }

    // 最小公倍数
    let mut m: u128;
    m = (a[0] * a[1]) / gcd(a[0], a[1]);
    for i in 2..n {
        m = (m * a[i as usize ]) / gcd(m, a[i as usize ]);
    }
    println!("{}", m)

}

fn gcd(a: u128, b: u128) -> u128 {
    let mut r: u128 = 0;

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

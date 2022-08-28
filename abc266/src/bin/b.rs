use proconio::input;

fn main() {
    input! {
        n: isize
    }

    // if n >= 0 {
    //     let sho = n / 998244353;

    //     println!("{}", n - 998244353 * sho)
    // } else {
    println!("{}", mod_minus(n, 998244353))
    // }
}

fn mod_minus(val: isize, m: isize) -> usize {
    let mut res = val % m;
    if res < 0 {
        res += m;
    }
    return res as usize;
}

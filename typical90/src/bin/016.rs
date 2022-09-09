use proconio::input;

fn main() {
    input! {
        n: u128,
        a: u128,
        b: u128,
        c: u128
    }

    let mut count = std::u128::MAX;
    let aaa = n / a;
    let bbb = n / b;
    for aa in 0..=aaa {
        for bb in 0..=bbb {
            if a * aa + b * bb > n {
                break;
            }
            let cc = (n - (a * aa + b * bb)) / c;
            if a * aa + b * bb + c * cc == n {
                count = count.min(aa + bb + cc)
            }
        }
    }

    println!("{}", count)
}

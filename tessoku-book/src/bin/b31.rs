use proconio::input;

fn main() {
    input! {
        n: usize
    }

    let a1 = n / 3;
    let a2 = n / 5;
    let a3 = n / 7;

    let b1 = n / 21;
    let b2 = n / 15;
    let b3 = n / 35;

    let c1 = n / 105;

    println!("{}", a1 + a2 + a3 - (b1 + b2 + b3) + c1)
}

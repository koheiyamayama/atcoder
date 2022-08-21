use proconio::input;

fn main() {
    input! {
        n: usize
    }

    let maisu = n / 1000;
    let amari = n % 1000;
    if amari == 0 {
        println!("0")
    } else {
        println!("{}", (maisu + 1) * 1000 - n)
    }
}

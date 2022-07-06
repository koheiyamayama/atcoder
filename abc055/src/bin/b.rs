use proconio::input;

fn main() {
    input! {
        n: u128
    }
    
    let m: u128 = 10u128.pow(9u32) + 7;
    let mut result: u128 = 1;
    for i in 1..=n {
        result *= i;
        result %= m;
    }

    
    println!("{}", result);
}

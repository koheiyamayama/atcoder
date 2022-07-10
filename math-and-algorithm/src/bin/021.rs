use proconio::input;

fn main() {
    input! {
        n: usize,
        r: usize
    }

    println!("{}", factorial(n) / (factorial(r) * factorial(n-r)))
}

fn factorial(n: usize) -> usize {
    let mut result: usize = 1;
    for i in 1..=n {
        result *= i;
    }

    return result
}

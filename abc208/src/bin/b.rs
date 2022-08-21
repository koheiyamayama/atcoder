use proconio::input;

fn main() {
    input! {
        mut p: usize
    }

    let mut count = 0;
    for i in 1..=10 {
        count += p / factorial(11 - i);
        p %= factorial(11 - i);
    }

    println!("{}", count)
}

fn factorial(n: usize) -> usize {
    let mut result: usize = 1;
    for i in 1..=n {
        result *= i;
    }

    return result;
}

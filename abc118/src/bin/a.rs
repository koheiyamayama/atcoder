use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize
    }

    if b % a == 0 {
        println!("{}", a + b)
    } else {
        println!("{}", b - a)
    }
}

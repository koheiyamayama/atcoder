use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize
    }

    if a + b < 24 {
        println!("{}", a + b)
    } else {
        println!("{}", a + b - 24)
    }
}

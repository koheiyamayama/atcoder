use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize
    }

    if a <= 5 {
        println!("{}", 0)
    } else if 6 <= a && a <= 12 {
        println!("{}", b / 2)
    } else {
        println!("{}", b)
    }
}

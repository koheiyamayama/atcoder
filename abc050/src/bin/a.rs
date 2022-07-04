use proconio::input;

fn main() {
    input! {
        a: isize,
        op: char,
        b: isize,
    }

    if op == '+' {
        println!("{}", a+b)
    } else if op == '-' {
        println!("{}", a-b)
    }
}

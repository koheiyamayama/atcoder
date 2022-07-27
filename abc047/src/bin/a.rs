use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize
    }

    if a + b == c {
        println!("Yes")
    } else if a + c == b {
        println!("Yes")
    } else if b + c == a {
        println!("Yes")
    } else {
        println!("No")
    }
}

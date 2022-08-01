use proconio::input;

fn main() {
    input! {
        mut a: usize,
        mut b: usize
    }

    a = if a == 1 { 14 } else { a };
    b = if b == 1 { 14 } else { b };

    if a < b {
        println!("Bob")
    } else if a > b {
        println!("Alice")
    } else {
        println!("Draw")
    }
}

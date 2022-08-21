use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize
    }

    if n % 500 == 0 || n % 500 <= a {
        println!("Yes")
    } else {
        println!("No")
    }
}

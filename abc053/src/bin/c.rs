use proconio::input;

fn main() {
    input! {
        x: usize
    }

    if x <= 6 {
        println!("1")
    } else if x <= 11 {
        println!("2")
    } else {
        if x % 11 == 0 {
            println!("{}", x / 11 * 2)
        } else if x % 11 <= 6 {
            println!("{}", x / 11 * 2 + 1)
        } else {
            println!("{}", x / 11 * 2 + 2)
        }
    }
}

use proconio::input;

fn main() {
    input! {
        a: isize,
        b: isize,
        k: isize
    }

    if b - a + 1 >= 2 * k {
        for i in a..a + k {
            println!("{}", i)
        }

        for i in b - k + 1..=b {
            println!("{}", i)
        }
    } else {
        for i in a..=b {
            println!("{}", i)
        }
    }
}

use proconio::input;

fn main() {
    input! {
        n: isize
    }

    println!("{}", n *800 - (n / 15) * 200)
}

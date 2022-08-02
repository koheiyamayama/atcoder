use proconio::input;

fn main() {
    input! {
        a: isize,
        b: isize,
        c: isize
    }

    if b - a == c - b {
        println!("YES")
    } else {
        println!("NO")
    }
}

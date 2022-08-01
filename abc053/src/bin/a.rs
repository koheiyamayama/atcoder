use proconio::input;

fn main() {
    input! {
        r: usize
    }

    if r < 1200 {
        println!("ABC")
    } else {
        println!("ARC")
    }
}

use proconio::input;

fn main() {
    input! {
        x: usize
    }

    let mut p: usize = 0;
    let mut t: usize = 0;
    while p < x {
        p += t;
        t += 1;
    }

    println!("{}", t - 1)
}

use proconio::input;

fn main() {
    input! {
        a: isize,
        b: isize
    }

    let mut num = -1;
    for i in 1..=1_000 {
        if (i as f64 * 0.08).floor() as isize == a && (i as f64 * 0.1).floor() as isize == b {
            num = i;
            break;
        }
    }

    println!("{}", num)
}

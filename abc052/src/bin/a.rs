use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize
    }

    let e = a * b;
    let f = c * d;
    if e <= f {
        println!("{}", f)
    } else {
        println!("{}", e)
    }
}

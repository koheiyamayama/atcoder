use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize
    }

    let d = a + b;
    let e = a + c;
    let f = b + c;

    println!("{}", vec![d, e, f].iter().min().unwrap())
}

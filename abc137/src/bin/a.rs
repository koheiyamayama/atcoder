use proconio::input;

fn main() {
    input! {
        a: isize,
        b: isize
    }

    let v: Vec<isize> = vec![a + b, a - b, a * b];
    println!("{}", v.iter().max().unwrap());
}

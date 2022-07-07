use proconio::input;
use std::cmp;

fn main() {
    input! {
        a: i64,
        b: i64
    }

    let c = cmp::max(a+b, a-b);
    
    let r = cmp::max(c, a*b);

    println!("{}", r)
}

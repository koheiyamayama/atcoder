use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize
    }

    let d = a * 100 + b * 10 + c;
    if d % 4 == 0 {
        println!("YES")
    } else {
        println!("NO")
    }
}

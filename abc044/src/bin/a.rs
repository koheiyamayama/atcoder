use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        x: usize,
        y: usize
    };

    if n <= k {
        println!("{}", n * x);
    } else {
        println!("{}", k*x+(n-k)*y)
    }
}

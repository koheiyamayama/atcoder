use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize
    }

    let mut result = k;
    for _i in 0..n-1 {
        result *= k -1;
    }

    println!("{}", result)
}

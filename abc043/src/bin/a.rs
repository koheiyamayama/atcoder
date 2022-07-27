use proconio::input;

fn main() {
    input! {
        n: usize
    }

    let mut sum: usize = 0;

    for i in 0..n {
        sum += i+1;
    }

    println!("{}", sum)
}

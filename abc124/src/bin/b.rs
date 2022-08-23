use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [usize; n]
    }

    let mut count = 0;
    let mut heighest = h[0];
    for i in 0..n {
        if heighest <= h[i] {
            count += 1;
            heighest = h[i];
        }
    }

    println!("{}", count)
}

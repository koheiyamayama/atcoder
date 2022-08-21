use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        l: [usize; n]
    }

    let mut count = 1;
    let mut acum = 0;
    for i in 0..n {
        acum += l[i];
        if acum <= x {
            count += 1;
        }
    }

    println!("{}", count)
}

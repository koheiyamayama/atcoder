use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
        n: usize,
    }

    let mut yen = 0;
    let x_per = x;
    let y_per = y / 3;

    if x_per <= y_per {
        yen = n * x;
    } else {
        yen = y * (n / 3) + x * (n % 3);
    }

    println!("{}", yen)
}

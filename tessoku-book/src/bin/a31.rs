use proconio::input;

fn main() {
    input! {
        n: usize
    }

    let three = n / 3;
    let five = n / 5;
    let fifteen = n / 15;

    println!("{}", three + five - fifteen);
}

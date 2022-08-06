use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n]
    }

    a.sort();

    let mut result = 0;
    for i in 0..n {
        if i == 0 {
            continue;
        } else {
            result = result + a[i] - a[i - 1];
        }
    }

    println!("{}", result)
}

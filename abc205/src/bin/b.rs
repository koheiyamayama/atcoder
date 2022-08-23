use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n]
    }

    a.sort();
    let mut b = vec![0; n];
    for i in 0..n {
        b[i] = i + 1;
    }

    if a == b {
        println!("Yes")
    } else {
        println!("No")
    }
}

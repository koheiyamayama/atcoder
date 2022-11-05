use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Usize1; m]
    }

    let mut members = vec![0; n];
    for i in 0..m {
        members[a[i]] += 1;
    }

    for i in 0..n {
        println!("{}", m - members[i])
    }
}

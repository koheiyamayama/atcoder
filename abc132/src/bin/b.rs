use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n]
    }

    let mut count = 0;
    for i in 1..n - 1 {
        let before = p[i - 1];
        let middle = p[i];
        let next = p[i + 1];

        if (before < middle && middle < next) || (next < middle && middle < before) {
            count += 1;
        }
    }

    println!("{}", count)
}

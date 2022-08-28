use proconio::input;

fn main() {
    input! {
        s: usize,
        t: usize
    }

    let mut count = 0;
    for i in 0..101 {
        for j in 0..101 {
            for k in 0..101 {
                if i + j + k <= s && i * j * k <= t {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count)
}

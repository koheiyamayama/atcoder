use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n-1]
    }

    let mut c = p[n - 2];
    let mut count = 1;
    while true {
        if c == 1 {
            break;
        }
        c = p[c - 2];
        count += 1;
    }

    println!("{}", count)
}

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut count = 1;
    let mut next = 1;
    while true {
        if 2 == a[next - 1] {
            println!("{}", count);
            break;
        }
        count += 1;
        next = a[next - 1];
        if count >= 10000000 {
            println!("-1");
            break;
        }
    }
}

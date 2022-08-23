use proconio::input;

fn main() {
    input! {
        n: isize,
        x: isize,
        m: [isize; n]
    }

    let mut count = n;
    let mut x: isize = x - m.iter().sum::<isize>();
    let min: isize = *m.iter().min().unwrap();

    while x >= min {
        count += 1;
        x -= min;
    }

    println!("{}", count)
}

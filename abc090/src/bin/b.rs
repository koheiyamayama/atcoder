use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize
    }

    let mut count = 0;
    for i in a..=b {
        let one = i / 10_000;
        let two = i % 10_000 / 1_000;
        let four = i % 10_000 % 1_000 % 100 / 10;
        let five = i % 10_000 % 1_000 % 100 % 10 / 1;
        // println!("one: {}, two: {}, four: {}, five: {}", one, two, four, five);
        if one == five && two == four {
            count += 1;
        }
    }

    println!("{}", count)
}

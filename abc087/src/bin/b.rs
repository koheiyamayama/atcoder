use proconio::input;

fn main() {
    input! {
        // 500yen
        a: i32,
        // 100yen
        b: i32,
        // 50yen
        c: i32,
        // count
        x: i32
    }

    let mut count = 0;

    for five_hundred in 0..=a {
        if five_hundred * 500 > x {
            continue;
        }
        for one_hundred in 0..=b {
            if five_hundred * 500 + one_hundred * 100 > x {
                continue;
            }
            for fifty in 0..=c {
                if five_hundred * 500 + one_hundred * 100 + fifty * 50 == x {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);
}

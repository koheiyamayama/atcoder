use proconio::input;

fn main() {
    input! {
        k: isize,
        s: isize
    }

    let mut count: isize = 0;

    for x in 0..=k {
        for y in 0..=k {
            let z = s - x - y;
            if 0 <= z && z <= k {
                count += 1;
            }
        }
    }

    println!("{}", count)
}

// x + y + z = s
// 0 + 1 + z = 2
// z = 2 - 1 - 0

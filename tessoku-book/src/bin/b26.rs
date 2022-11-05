use proconio::input;

fn main() {
    input! {
        n: usize
    }

    let mut deleted = vec![false; n + 1];
    let root = (n as f64).sqrt().floor() as usize;
    deleted[0] = true;
    deleted[1] = true;

    for i in 2..=root {
        if deleted[i] {
            continue;
        }

        for j in 2..=n {
            if i * j <= n {
                deleted[i * j] = true;
            }
        }
    }

    for i in 0..=n {
        if deleted[i] {
        } else {
            println!("{}", i)
        }
    }
}

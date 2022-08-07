use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        e: usize
    }

    let mut results = vec![0; 14];

    for i in [a, b, c, d, e].iter() {
        results[*i] += 1;
    }

    let mut is_three = false;
    let mut is_two = false;

    for r in results {
        if r == 3 {
            is_three = true;
        }

        if r == 2 {
            is_two = true;
        }
    }

    if is_three && is_two {
        println!("Yes")
    } else {
        println!("No")
    }
}

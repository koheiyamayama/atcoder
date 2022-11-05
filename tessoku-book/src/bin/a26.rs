use proconio::input;

fn main() {
    input! {
        q: usize,
        x: [usize; q]
    }

    for i in 0..q {
        if is_prime(x[i]) {
            println!("Yes")
        } else {
            println!("No")
        }
    }
}

fn is_prime(x: usize) -> bool {
    let r = (x as f64).sqrt().floor() as usize;
    for i in 2..=r {
        if x % i == 0 {
            return false;
        }
    }

    return true;
}

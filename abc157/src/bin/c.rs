use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        sc: [(Usize1, usize); m]
    }

    let mut digits = vec![std::usize::MAX; n];
    for (s, c) in sc {
        if digits[s] != std::usize::MAX && digits[s] != c {
            println!("-1");
            return;
        }

        digits[s] = c;
    }

    if digits[0] == 0 && n > 1 {
        println!("-1");
        return;
    }

    if digits[0] == std::usize::MAX && n > 1 {
        digits[0] = 1;
    }

    for i in 0..n {
        if digits[i] == std::usize::MAX {
            print!("0")
        } else {
            print!("{}", digits[i])
        }
    }

    println!("")
}

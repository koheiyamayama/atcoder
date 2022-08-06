use proconio::input;

fn main() {
    input! {
        n: usize,
        mut s: [usize; n]
    }

    let result: usize = s.iter().sum();

    if result % 10 != 0 {
        println!("{}", result)
    } else {
        let mut smallest = std::usize::MAX;
        for score in s {
            if score % 10 != 0 {
                smallest = min(smallest, score);
            }
        }
        if smallest == std::usize::MAX {
            println!("{}", 0)
        } else {
            println!("{}", result - smallest)
        }
    }
}

fn min(n: usize, m: usize) -> usize {
    if n <= m {
        return n;
    } else {
        return m;
    }
}

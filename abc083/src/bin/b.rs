use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize
    }

    let mut sum = 0;
    for i in 1..=n {
        let s = digit_sum(i);
        if a <= s && s <= b {
            sum += i;
        }
    }

    println!("{}", sum)
}

fn digit_sum(mut n: usize) -> usize {
    let mut sum = 0;
    while n != 0 {
        sum += n % 10;
        n = n / 10;
    }

    return sum;
}

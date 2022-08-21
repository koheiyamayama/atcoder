use proconio::input;

fn main() {
    input! {
        k: usize,
        a: usize,
        b: usize
    }

    println!(
        "{}",
        convert_n_base_to_ten_base(k, a) * convert_n_base_to_ten_base(k, b)
    )
}

fn convert_n_base_to_ten_base(base: usize, mut n: usize) -> usize {
    let mut res = 0;
    let mut temp = 1;
    while n > 0 {
        res += n % 10 * temp;
        temp *= base;
        n /= 10;
    }
    res
}

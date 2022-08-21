use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize
    }

    let square = h * w;
    if is_even(h) && is_even(w) {
        println!("{}", square / 4)
    } else if (is_even(h) && is_odd(w)) || (is_even(w) && is_odd(h)) {
    } else {
    }
}

fn is_even(n: usize) -> bool {
    return n % 2 == 0;
}

fn is_odd(n: usize) -> bool {
    return !is_even(n);
}

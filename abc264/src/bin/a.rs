use proconio::input;

fn main() {
    input! {
        l: usize,
        r: usize
    }

    let atcoder = ['a', 't', 'c', 'o', 'd', 'e', 'r'];
    for (i, c) in atcoder.iter().enumerate() {
        if (l <= i + 1) && (i + 1 <= r) {
            print!("{}", c)
        }
    }

    println!("")
}

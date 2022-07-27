use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        mut s: [String; n]
    }

    s.sort();
    let s = s.join("");
    println!("{}", s)
}

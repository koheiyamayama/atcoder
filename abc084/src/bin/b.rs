use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut a: usize,
        mut b: usize,
        s: Chars
    }
    a -= 1;
    b -= 1;
    for i in 0..s.len() {
        if i <= a && !s[i].is_numeric() {
            println!("No");
            return;
        } else if i == a + 1 && !(s[i] == '-') {
            println!("No");
            return;
        } else if a + 2 <= i && !s[i].is_numeric() {
            println!("No");
            return;
        }
    }

    println!("Yes")
}

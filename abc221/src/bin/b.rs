use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars
    }

    for i in 0..s.len() - 1 {
        let mut cloned = s.clone();
        if cloned == t {
            println!("Yes");
            return;
        }
        cloned.swap(i, i + 1);
        if cloned == t {
            println!("Yes");
            return;
        }
    }

    println!("No")
}

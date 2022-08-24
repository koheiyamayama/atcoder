use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars
    }

    let mut cloned = s.clone();
    for i in 0..s.len() {
        let c = cloned.pop().unwrap();
        cloned.insert(0, c);
        if t == cloned {
            println!("Yes");
            return;
        }
    }

    println!("No")
}

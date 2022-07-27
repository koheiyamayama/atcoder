use proconio::input;
use proconio::marker::{Chars};

fn main() {
    input! {
        mut a: Chars,
        mut b: Chars,
        mut c: Chars
    }

    let mut flag = true;
    let mut d = a.remove(0);
    while flag {
        if d == 'a' {
            if a.len() == 0 {
                println!("A");
                break;
            } else {
                d = a.remove(0);
            }
        } else if d == 'b' {
            if b.len() == 0 {
                println!("B");
                break;
            } else {
                d = b.remove(0);
            }
        } else if d == 'c' {
            if c.len() == 0 {
                println!("C");
                break;
            } else {
                d = c.remove(0);
            }
        }
    }
}

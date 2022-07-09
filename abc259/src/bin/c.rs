use std::vec;
use proconio::input;

fn main() {
    input! {
        mut s: String,
        mut t: String
    }

    let s = trans(s);
    let t = trans(t);

    let mut check = s.len() == t.len();

    for (m , n)in s.iter().zip(t) {
        if m.0 != n.0 {
            check = false;
            break;
        }

        if (m.1 < n.1 && m.1 == 1) || m.1 > n.1 {
            check = false;
            break;
        }
    }

    if check {
        println!("Yes")
    } else {
        println!("No")
    }
}

fn trans(string: String) -> Vec<(char, usize)> {
    let mut v = vec![];

    for c in string.chars() {
        match v.last_mut() {
            None => {
                v.push((c, 1))
            },
            Some((k, n)) => {
                if c == *k {
                    *n += 1;
                } else {
                    v.push((c, 1))
                }
            }
        }
    }

    return v
}

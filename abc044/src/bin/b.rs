use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        w: String
    }

    let mut t: HashMap<char, usize> = HashMap::new();

    for c in w.chars() {
        match t.get(&c) {
            Some(v) => t.insert(c, *v+1),
            None => t.insert(c, 1),
        };
    }

    let mut r = true;
    for (_k, v) in &t {
        if v % 2 != 0 {
            r = false;
            break;
        }
    }
    if r {
        println!("Yes")
    } else {
        println!("No")
    }
}

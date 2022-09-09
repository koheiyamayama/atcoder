use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n]
    }

    let mut map: HashMap<String, usize> = HashMap::new();
    for i in 0..n {
        match map.get_mut(&s[i].to_string()) {
            Some(v) => continue,
            None => {
                map.insert(s[i].to_string(), 1);
                println!("{}", i + 1);
            }
        }
    }
}

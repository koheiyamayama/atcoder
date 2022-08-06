use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }

    let mut counter = vec![0; 26];

    for c in s {
        let target_alphabet = (c as u8 - 'a' as u8) as usize;
        counter[target_alphabet] += 1;
    }

    let mut flag = true;
    for c in counter {
        if c > 1 {
            flag = false;
        }
    }

    if flag {
        println!("yes")
    } else {
        println!("no")
    }
}

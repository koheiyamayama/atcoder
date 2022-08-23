use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }

    // println!("{}", 'a' as u8);
    // println!("{}", 'z' as u8);
    // println!("{}", 'A' as u8);
    // println!("{}", 'Z' as u8);

    // 先頭A
    let a_flag = s[0] == 'A';

    // 3~-2までにCが一文字
    let mut c_count = 0;
    for i in 2..s.len() - 1 {
        if s[i] == 'C' {
            c_count += 1;
        }
    }
    let c_flag = c_count == 1;

    // 全部小文字
    let mut s_flag = true;
    for i in 1..s.len() {
        if s[i] == 'A' || s[i] == 'C' {
            continue;
        }
        if s[i].is_ascii_uppercase() {
            s_flag = false;
            break;
        }
    }

    if a_flag && c_flag && s_flag {
        println!("AC")
    } else {
        println!("WA")
    }
}

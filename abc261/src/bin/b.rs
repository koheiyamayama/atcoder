use proconio::input;
use proconio::marker::{Chars};

fn main() {
    input! {
        n: usize,
        a: [Chars; n]
    }

    let mut flag = true;
    for i in 0..n {
        for m in 0..n {
            // println!("a[{}][{}]: {}, a[{}][{}]: {}", i, m, a[i][m], m, i, a[m][i]);
            if !is_correct(a[i][m], a[m][i]) {
                flag = false;
            }
        }
    }

    if flag {
        println!("correct")
    } else {
        println!("incorrect")
    }
}

fn is_correct(c1: char, c2: char) -> bool {
    if c1 == 'W' && c2 == 'L' {
        return true
    } else if c1 == 'L' && c2 == 'W' {
        return true
    } else if c1 == 'D' && c2 == 'D' {
        return true
    } else if c1 == '-' && c2 == '-' { 
        return true
    } else {
        return false
    }
}

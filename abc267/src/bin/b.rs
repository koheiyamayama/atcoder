use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }

    if s[0] == '1' {
        println!("No");
        return;
    }

    let mut rows = vec![false; 7];
    rows[0] = s[6] == '1';
    rows[1] = s[3] == '1';
    rows[2] = s[1] == '1' || s[7] == '1';
    rows[3] = s[0] == '1' || s[4] == '1';
    rows[4] = s[2] == '1' || s[8] == '1';
    rows[5] = s[5] == '1';
    rows[6] = s[9] == '1';

    for i in 0..7 {
        for j in i + 1..7 {
            if rows[i] && !rows[j] && rows[j..].contains(&true) {
                println!("Yes");
                return;
            }
        }
    }

    println!("No")
}

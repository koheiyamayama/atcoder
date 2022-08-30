use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut s: [Chars; n]
    }

    for i in (0..n).rev() {
        for j in (0..2 * n - 1).rev() {
            if s[i][j] == '#' && i != n - 1 {
                // 左下
                if s[i + 1][j - 1] == 'X' {
                    s[i][j] = 'X';
                // 下
                } else if s[i + 1][j] == 'X' {
                    s[i][j] = 'X';
                // 右下
                } else if s[i + 1][j + 1] == 'X' {
                    s[i][j] = 'X';
                } else {
                    s[i][j] = '#';
                }
            }
        }
    }

    for i in 0..n {
        for j in 0..2 * n - 1 {
            print!("{}", s[i][j])
        }
        println!("")
    }
}

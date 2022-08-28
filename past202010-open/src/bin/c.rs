use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Chars; h]
    }

    let mut g = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 0..w {
            // 上
            if i != 0 && a[i - 1][j] == '#' {
                g[i][j] += 1;
            }

            // 右上
            if i != 0 && j != w - 1 && a[i - 1][j + 1] == '#' {
                g[i][j] += 1;
            }

            // 右
            if j != w - 1 && a[i][j + 1] == '#' {
                g[i][j] += 1;
            }

            // 右下
            if i != h - 1 && j != w - 1 && a[i + 1][j + 1] == '#' {
                g[i][j] += 1;
            }

            // 下
            if i != h - 1 && a[i + 1][j] == '#' {
                g[i][j] += 1;
            }

            // 左下
            if i != h - 1 && j != 0 && a[i + 1][j - 1] == '#' {
                g[i][j] += 1;
            }

            // 左
            if j != 0 && a[i][j - 1] == '#' {
                g[i][j] += 1;
            }

            // 左上
            if i != 0 && j != 0 && a[i - 1][j - 1] == '#' {
                g[i][j] += 1;
            }

            if a[i][j] == '#' {
                g[i][j] += 1;
            }
        }
    }

    for i in 0..h {
        for j in 0..w {
            print!("{}", g[i][j])
        }
        println!("")
    }
}

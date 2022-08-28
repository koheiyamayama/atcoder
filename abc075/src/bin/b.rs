use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h]
    }

    let mut new_s = vec![vec![0; w]; h];

    for i in 0..h {
        for j in 0..w {
            let mut count = 0;
            if s[i][j] == '#' {
                count = std::isize::MAX;
                new_s[i][j] = count;

                continue;
            }
            // // 左斜め上
            // if i > 0 && j > 0 && s[i - 1][j - 1] == '#' {
            //     count += 1;
            // }
            // // 上
            // if i > 0 && s[i - 1][j] == '#' {
            //     count += 1;
            // }
            // // 右斜め上
            // if i > 0 && j != w - 1 && s[i - 1][j + 1] == '#' {
            //     count += 1;
            // }
            // // 右
            // if j != w - 1 && s[i][j + 1] == '#' {
            //     count += 1;
            // }
            // // 右斜下
            // if i + 1 < h && j != w - 1 && s[i + 1][j + 1] == '#' {
            //     count += 1;
            // }
            // // 下
            // if i + 1 < h && s[i + 1][j] == '#' {
            //     count += 1;
            // }
            // // 左斜した
            // if i + 1 < h && j > 0 && s[i + 1][j - 1] == '#' {
            //     count += 1;
            // }
            // // 左
            // if j > 0 && s[i][j - 1] == '#' {
            //     count += 1;
            // }

            // 左斜め上
            if i > 0 && j > 0 && s[i - 1][j - 1] == '#' {
                count += 1;
            }
            // 上
            if i > 0 && s[i - 1][j] == '#' {
                count += 1;
            }
            // 右斜め上
            if i > 0 && j != w - 1 && s[i - 1][j + 1] == '#' {
                count += 1;
            }
            // 右
            if j != w - 1 && s[i][j + 1] == '#' {
                count += 1;
            }
            // 右斜下
            if i + 1 < h && j != w - 1 && s[i + 1][j + 1] == '#' {
                count += 1;
            }
            // 下
            if i + 1 < h && s[i + 1][j] == '#' {
                count += 1;
            }
            // 左斜した
            if i + 1 < h && j > 0 && s[i + 1][j - 1] == '#' {
                count += 1;
            }
            // 左
            if j > 0 && s[i][j - 1] == '#' {
                count += 1;
            }

            new_s[i][j] = count;
        }
    }

    for i in 0..h {
        for j in 0..w {
            if new_s[i][j] == std::isize::MAX {
                print!("#")
            } else {
                print!("{}", new_s[i][j])
            }
        }
        println!("")
    }
}

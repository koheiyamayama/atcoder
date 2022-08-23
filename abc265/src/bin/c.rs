use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        g: [Chars; h]
    }

    let limit = 1000000;
    let mut i = 0;
    let mut j = 0;
    let mut before = (i, j);

    let mut index = 0;

    let mut flag = true;
    while true {
        if g[i][j] == 'U' && i != 0 {
            i -= 1;
        } else if g[i][j] == 'D' && i != h - 1 {
            i += 1;
        } else if g[i][j] == 'L' && j != 0 {
            j -= 1;
        } else if g[i][j] == 'R' && j != w - 1 {
            j += 1;
        }

        if before == (i, j) {
            break;
        } else {
            before = (i, j);
        }

        if index >= limit {
            flag = false;
            break;
        }
        index += 1;
    }

    if flag {
        println!("{} {}", i + 1, j + 1)
    } else {
        println!("-1")
    }
}

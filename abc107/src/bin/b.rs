use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Chars; h]
    }

    // 行
    let mut rows: Vec<usize> = Vec::new();
    let mut columns: Vec<usize> = Vec::new();
    for i in 0..h {
        if !a[i].contains(&'#') {
            rows.push(i);
        }
    }

    for i in 0..w {
        let mut flag = true;
        for j in 0..h {
            if a[j][i] == '#' {
                flag = false;
                break;
            }
        }

        if flag {
            columns.push(i)
        }
    }

    for i in 0..h {
        if rows.contains(&i) {
            continue;
        }
        for j in 0..w {
            if columns.contains(&j) {
                continue;
            } else {
                print!("{}", a[i][j])
            }
        }
        println!("")
    }
}

use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        h: usize,
        w: usize,
        mut y: usize,
        mut x: usize,
        s: [Chars; h]
    }

    x -= 1;
    y -= 1;

    let mut top = 0;
    for yy in (0..y).rev() {
        if s[yy][x] == '#' {
            break;
        } else {
            top += 1;
        }
    }

    let mut bottom = 0;
    for yy in y + 1..h {
        if s[yy][x] == '#' {
            break;
        } else {
            bottom += 1;
        }
    }

    let mut right = 0;
    for xx in x + 1..w {
        if s[y][xx] == '#' {
            break;
        } else {
            right += 1;
        }
    }

    let mut left = 0;
    for xx in (0..x).rev() {
        if s[y][xx] == '#' {
            break;
        } else {
            left += 1;
        }
    }

    println!("{}", top + bottom + right + left + 1)
}

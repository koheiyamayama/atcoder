use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        x: [[usize; w]; h],
        q: usize,
        a: [(usize, usize, usize, usize); q]
    }

    let mut acum: Vec<Vec<usize>> = vec![vec![0; w + 1]; h];
    for i in 0..h {
        for j in 0..w {
            acum[i][j + 1] = x[i][j] + acum[i][j];
        }
    }

    for i in 0..w + 1 {
        for j in 1..h {
            acum[j][i] = acum[j - 1][i] + acum[j][i];
        }
    }
    acum.insert(0, vec![0; w + 1]);

    for i in 0..q {
        let y = a[i].0;
        let x = a[i].1;
        let yy = a[i].2;
        let xx = a[i].3;
        println!(
            "{}",
            acum[y - 1][x - 1] + acum[yy][xx] - acum[y - 1][xx] - acum[yy][x - 1]
        )
    }
}

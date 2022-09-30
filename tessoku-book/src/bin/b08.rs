use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(usize, usize); n],
        q: usize,
        abcd: [(usize, usize, usize, usize); q]
    }

    let mut acum = vec![vec![0; 1501]; 1501];
    for i in 0..n {
        acum[xy[i].1][xy[i].0] += 1;
    }

    for i in 0..1501 {
        for j in 1..1501 {
            acum[i][j] += acum[i][j - 1];
        }
    }

    for i in 1..1501 {
        for j in 0..1501 {
            acum[i][j] += acum[i - 1][j];
        }
    }

    for i in 0..q {
        let a = abcd[i].0;
        let b = abcd[i].1;
        let c = abcd[i].2;
        let d = abcd[i].3;

        println!(
            "{}",
            acum[d][c] - acum[b - 1][c] - acum[d][a - 1] + acum[b - 1][a - 1]
        );
    }
}

use proconio::input;

fn main() {
    input! {
        n: usize,
        abcd: [(usize, usize, usize, usize); n]
    }

    let mut acum = vec![vec![0; 1502]; 1502];
    for i in 0..n {
        let a = abcd[i].0;
        let b = abcd[i].1;
        let c = abcd[i].2;
        let d = abcd[i].3;

        acum[b + 1][a + 1] += 1;
        acum[d + 1][c + 1] += 1;

        acum[b + 1][c + 1] -= 1;
        acum[d + 1][a + 1] -= 1;
    }

    for i in 0..1502 {
        for j in 1..1502 {
            acum[i][j] = acum[i][j - 1] + acum[i][j]
        }
    }

    for i in 1..1502 {
        for j in 0..1502 {
            acum[i][j] = acum[i - 1][j] + acum[i][j]
        }
    }

    let mut ans = 0;
    for i in 0..1502 {
        for j in 0..1502 {
            if acum[i][j] > 0 {
                ans += 1;
            }
        }
    }

    println!("{}", ans)
}

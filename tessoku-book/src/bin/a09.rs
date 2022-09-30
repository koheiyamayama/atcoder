use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        abcd: [(usize, usize, usize, usize); n]
    }

    let mut table_one = vec![vec![0; w + 2]; h + 2];

    for i in 0..n {
        let a = abcd[i].0;
        let b = abcd[i].1;
        let c = abcd[i].2;
        let d = abcd[i].3;

        table_one[a][b] += 1;
        table_one[c + 1][d + 1] += 1;
        table_one[c + 1][b] -= 1;
        table_one[a][d + 1] -= 1;
    }

    let mut table = vec![vec![0; w + 1]; h + 1];

    for i in 1..h + 1 {
        for j in 1..w + 1 {
            table[i][j] = table_one[i][j] + table[i][j - 1]
        }
    }

    for i in 1..h + 1 {
        for j in 1..w + 1 {
            table[i][j] = table[i][j] + table[i - 1][j]
        }
    }

    for i in 1..h + 1 {
        for j in 1..w + 1 {
            if j != w {
                print!("{:?} ", table[i][j])
            } else {
                print!("{:?}", table[i][j])
            }
        }
        println!("")
    }
}

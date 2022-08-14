use proconio::input;

fn main() {
    input! {
        h1: usize,
        w1: usize,
        a: [[usize; w1]; h1],
        h2: usize,
        w2: usize,
        b: [[usize; w2]; h2],
    }

    let mut grid: Vec<Vec<usize>> = vec![vec![0; w1]; h1];

    for i in 0..h1 {
        for m in 0..w1 {
            for j in 0..h2 {
                for k in 0..w2 {
                    if a[i][m] == b[j][k] {
                        grid[i][m] = a[i][m];
                        break;
                    }
                }
            }
        }
    }

    let mut new_grid: Vec<Vec<usize>> = Vec::new();
    for (i, g) in grid.iter().enumerate() {
        new_grid.push(vec![]);
        for h in g {
            if *h > 0 {
                new_grid[i].push(*h);
            }
        }
    }

    for g in new_grid {
        println!("{:?}", g)
    }
}

// Aの行列に対して、Bの行列がどのように当てはまるか？を考える。
// 当てはまり方が以下の条件を満たす時Yesといえる
// - 当てはまらないマスが列 or 行を満たす

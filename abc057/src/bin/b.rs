use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(isize, isize); n],
        cd: [(isize, isize); m]
    }

    // 最小の距離とその番号を持つtupleをvectorで持てばもっとシンプルに実装できる。
    // https://atcoder.jp/contests/abc057/submissions/32667467 これとか参考になる。
    let mut results: Vec<Vec<isize>> = vec![vec![std::isize::MAX; m]; n];

    for i in 0..n {
        for j in 0..m {
            results[i][j] = abs(ab[i].0 - cd[j].0) + abs(ab[i].1 - cd[j].1);
        }
    }

    for r in &results {
        let minimum = r.iter().min().unwrap();
        let mut i = 0;
        for t in r {
            if *t == *minimum {
                println!("{}", i + 1);
                break;
            }
            i += 1;
        }
    }
}

fn min(n: isize, m: isize) -> isize {
    if n <= m {
        return n;
    } else {
        return m;
    }
}

fn abs(n: isize) -> isize {
    if n < 0 {
        return n * -1;
    } else {
        return n;
    }
}

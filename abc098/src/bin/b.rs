use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars
    }

    let mut before = vec![vec![false; 26]; n];
    let mut after = vec![vec![false; 26]; n];

    for i in 0..n {
        // 前半
        for j in 0..i {
            before[i][s[j] as usize - 'a' as usize] = true;
        }

        // 後半
        for k in i..n {
            after[i][s[k] as usize - 'a' as usize] = true;
        }
    }

    let mut ans = 0;
    for i in 0..n {
        let mut count = 0;
        for j in 0..26 {
            if before[i][j] && after[i][j] {
                count += 1;
            }
        }

        ans = ans.max(count);
    }

    println!("{}", ans)
}

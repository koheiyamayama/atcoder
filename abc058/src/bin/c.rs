use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n]
    }

    let mut maps: Vec<Vec<usize>> = vec![vec![0; 26]; n];

    for i in 0..n {
        for j in 0..s[i].len() {
            maps[i][(s[i][j] as u8 - 97) as usize] += 1;
        }
    }

    let mut ans = vec![std::usize::MAX; 26];
    for i in 0..n {
        for j in 0..26 {
            ans[j] = ans[j].min(maps[i][j]);
        }
    }

    let mut flag = false;
    for i in 0..26 {
        if ans[i] != std::usize::MAX {
            for j in 0..ans[i] {
                print!("{}", (i + 97) as u8 as char);
                flag = true;
            }
        }
    }

    if flag {
        println!("")
    }
}

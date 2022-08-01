use std::collections::HashMap;

use proconio::input;

// 3つのtupleが以下の条件を満たすと三角形といえる
// 6つの数字が3種類、2つずつ存在すること
// N=100なので、3重ループを回せば解けるはず

fn main() {
    input! {
        n: usize,
        m: usize,
        lines: [(usize, usize); m]
    }

    let mut count = 0;

    for i in 0..n {
        for k in i + 1..n {
            for j in k + 1..n {
                let a = lines[i];
                let b = lines[k];
                let c = lines[j];

                println!("a: {:?}, b: {:?}, c: {:?}", a, b, c);
                if condition(a, b, c) {
                    count += 1;
                }
            }
        }
    }
    println!("{}", count)
}

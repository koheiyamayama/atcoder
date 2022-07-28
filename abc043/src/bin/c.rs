// こちら参考にした
// https://drken1215.hatenablog.com/entry/2020/10/29/024800

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [isize; n]
    }


    let mut result = std::isize::MAX;
    for x in -100..=100 {
        let mut tmp = 0;
        for i in 0..n {
            // a[i]の値を-100~100に書き換えるときのコストの総和を全探索する。
            // つまり、a[i]の値を-100に書き換えるときのコストの総和を計算する。
            tmp += (x - a[i]) * (x - a[i]);
        }
        // tmpには-100~100までのコストの総和があるので、-100のときと-99のときでどちらの総和が小さいのか比較する。
        // これを-100~100まで繰り返すことで最小のコストの総和が求められる。
        result = min(result, tmp);
    }

    println!("{}" , result)
}

fn min(n: isize, m: isize) -> isize {
    if n <= m {
        return n
    } else {
        return m
    }
}

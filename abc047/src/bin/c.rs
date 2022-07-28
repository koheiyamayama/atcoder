use std::collections::HashSet;

use proconio::input;
use proconio::marker::Chars;

fn main() {
    // input! {
    //     mut s: Chars
    // }

    // let mut count = 0;
    // let mut tmp: Vec<char> = Vec::new();
    // for c in s {
    //     if tmp.last() == None {
    //         tmp.push(c);
    //     } else if *tmp.last().unwrap() == c {
    //         tmp.push(c);
    //     } else {
    //         count += 1;
    //         tmp.push(c);
    //     }
    // }

    // println!("{}", count)

    // こちらの回答がわかりやすかったし、スマートだった
    // https://atcoder.jp/contests/abc047/submissions/32147630

    input! {
        mut s: Chars,
    }

    let mut result = 0;

    // 先頭の文字を取得
    let mut prev = s[0];

    for &c in &s {
        // 一つ前の文字と先頭から取り出した文字が異なる場合は
        // prevを更新し、resultをインクリメントする
        if prev == c {
            // do noting.
        } else {
            prev = c;
            result += 1;
        }
    }

    println!("{}", result);
}

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut h: [usize; n]
    }

    let mut res = 0;
    while true {
        // 最高の高さが0になっている場合、処理を終了する
        if *h.iter().max().unwrap() == 0 {
            break;
        }

        // 区間分割する
        let mut i = 0;
        while i < n {
            // h[i]が0の場合は何もしない
            // 区間を始めることができないから
            if h[i] == 0 {
                i += 1;
            // h[i]が0ではないということは、高さがあるので、区間を始める
            } else {
                // 区間が1つ以上あるので、変数をインクリメントする
                res += 1;
                // その区間を伸ばす
                // h[i]が0以下の場合は区間を閉じる
                // h[i]が1以上の場合は区間を広げることができる
                while i < n && h[i] > 0 {
                    h[i] -= 1;
                    i += 1;
                }
            }
        }
    }

    println!("{}", res)
}

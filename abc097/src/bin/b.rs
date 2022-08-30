use proconio::input;

fn main() {
    input! {
        x: usize
    }

    let mut ans = 1;

    // for b in 1..=x {
    //     for xx in 2..=10 {
    //         let tmp: u128 = b.pow(xx);
    //         if ans <= tmp && tmp <= x {
    //             ans = tmp;
    //         }
    //     }
    // }

    for i in 2..=x {
        // 以下のコードが何をやっているか
        // まず、i**2を求める
        // i**2がx以下の場合は答えになりうるので、探索をする
        // 探索時にそれまでの答えと比較して最も大きい数を答えとする
        // x=10の時
        // i=2の時には以下の順で計算をする
        // tmp = 2**2 = 4
        // 1回目のループ
        // ans = max(4, 1)
        // tmp = 4 * 2 = 8
        // 2回目のループ
        // ans = max(8, 4)
        // tmp = 8 * 2 = 16
        // ここでループが終わり、i = 3の計算をする
        // tmp = 3 ** 3;
        // 1回目のループ
        // ans = max(9, 8)
        // tmp = 9 * 3
        // これでループが終わる
        // これ以降はループが回ることはない 4**4はx以上
        let mut tmp = i * i;
        while tmp <= x {
            ans = tmp.max(ans);
            tmp *= i;
        }
    }

    println!("{}", ans)
}

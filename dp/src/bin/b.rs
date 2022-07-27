use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut h: [usize; n],
    }

    // dpには柱i-1から柱iに至るまでの最小コストを代入する
    let mut dp: Vec<usize> = vec![std::usize::MAX; n];
    // 柱0、つまり初期値はもともといる場所なので、移動コスト0
    dp[0] = 0;
    // 柱1に至るコストは柱0から移動してくるパターンしかない。
    dp[1] = abs_diff(h[0], h[1]);
    for i in 2..n {
        // 柱2以降は i-1, i-2,...i-jの柱から移動してくるパターンがあり、これらの移動パターンの中から最小のコストに移動元の柱と移動先の柱のコストを足せば良い。
        // min(h[i-1], h[i-2]..., i[i-j]) + dp[i-j]
        // 移動元の柱 = h[i-j]
        // 移動先の柱 = h[i]

        // 移動先の柱の一つ前に至るまでの最小コストと移動先の柱の高さと移動先の柱の高さの一つ前の高さの絶対値を足す
        // 要するに一つ前の柱が移動元の場合のコスト計算をしている。
        dp[i] = dp[i-1] + abs_diff(h[i], h[i-1]);
        for j in 2..k+1 {
            if i as isize - j as isize >= 0 {
                // i-jは移動元の座標
                // h[i-j]が移動元の柱の高さ
                // h[i]が移動先の柱の高さ
                // 移動元の柱の高さと移動先の柱の高さの絶対値を計算し、そこに移動元までに掛かった最小コストを足す。
                // その値とdp[i]つまり
                dp[i] = min(abs_diff(h[i-j], h[i]) + dp[i-j], dp[i]);
            }
        }
    }
    
    println!("{}", dp.last().unwrap())
}

fn abs_diff(n: usize, m: usize) -> usize {
    if n <= m {
        return m - n
    } else {
        return n - m
    }
}


fn min(n: usize, m: usize) -> usize {
    if n <= m {
        return n
    } else {
        return m
    }
}

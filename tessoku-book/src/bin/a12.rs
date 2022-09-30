use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n]
    }

    // プログラム自体は正しそうだが、おそらくansを計算するのにかなり時間がかかっていそう?TLEする。
    // let ans = (1..=1_000_000_000).collect_vec();
    // let result = ans
    //     .binary_search_by(|&x| {
    //         let mut sum = 0;
    //         for i in 0..n {
    //             sum += x / a[i];
    //         }

    //         if sum >= k {
    //             return Ordering::Greater;
    //         } else {
    //             return Ordering::Less;
    //         }
    //     })
    //     .unwrap_or_else(|i| i);

    // println!("{}", ans[result]);

    let mut l = 0;
    let mut r = 1_000_000_000;

    while l < r {
        let m = (l + r) / 2;
        let mut sum = 0;
        for i in 0..n {
            sum += m / a[i];
        }
        if sum >= k {
            r = m;
        } else {
            l = m + 1;
        }
    }

    println!("{}", l)
}

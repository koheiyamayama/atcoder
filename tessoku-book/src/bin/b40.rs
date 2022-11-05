use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
      n: usize,
      a: [usize; n]
    }

    let mut cnt: Vec<usize> = vec![0; 100];
    for i in 0..n {
        let m = a[i] % 100;
        cnt[m] += 1;
    }

    let mut ans = 0;

    for i in 1..50 {
        ans += cnt[i] * cnt[100 - i];
    }
    if cnt[50] > 0 {
        ans += cnt[50] * (cnt[50] - 1) / 2;
    }
    if cnt[0] > 0 {
        ans += cnt[0] * (cnt[0] - 1) / 2;
    }

    println!("{}", ans)
}

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: isize,
        ab: [(isize, isize); n]
    }

    let mut ans = 0;
    for a in 0..101 {
        for b in 0..101 {
            let mut tmp_ans = 0;
            for c in 0..n {
                if a as isize <= ab[c].0
                    && ab[c].0 <= a + k
                    && b as isize <= ab[c].1
                    && ab[c].1 <= b + k
                {
                    tmp_ans += 1;
                }
            }
            ans = ans.max(tmp_ans)
        }
    }

    println!("{}", ans)
}

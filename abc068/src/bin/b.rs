use proconio::input;

fn main() {
    input! {
        n: usize
    }

    let mut ans = vec![0; n + 1];
    for nn in 1..=n {
        let mut count = 0;
        let mut nnn = nn;
        while true {
            if nnn % 2 != 0 {
                break;
            }
            nnn = nnn / 2;
            count += 1;
        }
        ans[nn] = count;
    }

    let mut prev = 0;
    let mut tmp = 0;
    for (i, m) in ans.iter().enumerate() {
        if prev < *m {
            tmp = i;
            prev = *m;
        }
    }

    if n != 1 {
        println!("{:?}", tmp);
    } else {
        println!("1");
    }
}

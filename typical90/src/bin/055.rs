use proconio::input;

fn main() {
    input! {
        n: usize,
        p: usize,
        q: usize,
        a: [usize; n]
    }

    // let mut count = 1;
    // for _ in 0..20 {
    //     count *= 10;
    //     println!("{}", count)
    // }
    let mut count = 0;
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                for l in k + 1..n {
                    for m in l + 1..n {
                        if a[i] * a[j] * a[k] * a[l] * a[m] % p == q {
                            count += 1;
                        }
                    }
                }
            }
        }
    }

    println!("{}", count)
}

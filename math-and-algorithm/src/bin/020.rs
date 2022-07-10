use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut count = 0;
    for i in 0..n {
        for m in i + 1..n {
            for l in m + 1..n {
                for o in l + 1..n {
                    for p in o + 1..n {
                        let sum = a[i] + a[m] + a[l] + a[o] + a[p];
                        if sum == 1_000  {
                            count += 1;
                            // println!("{}, {}, {}, {}, {}", a[i], a[m], a[l], a[o], a[p])
                        }
                    }
                }
            }
        }
    }

    println!("{}", count)
}

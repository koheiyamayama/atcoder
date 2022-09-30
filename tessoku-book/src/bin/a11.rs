use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n]
    }

    // println!("{}", a.binary_search(&x).unwrap() + 1);
    // let mut l = 1;
    // let mut r = n;
    // let mut i = 1;
    // while l <= r {
    //     let m = (l + r) / 2;
    //     if x < a[m] {
    //         r = m - 1;
    //     } else if x == a[m] {
    //         println!("{}", m + 1);
    //         return;
    //     } else if x > a[m] {
    //         l = m + 1;
    //     }
    //     i += 1;
    // };

    let mut l = 0;
    let mut r = n;
    while l <= r {
        let mut m = (l + r) / 2;
        if a[m] > x {
            r = m - 1;
        } else if a[m] < x {
            l = m + 1;
        } else {
            println!("{}", m + 1);
            return;
        }
    }
}

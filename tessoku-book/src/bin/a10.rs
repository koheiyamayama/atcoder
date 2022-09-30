use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        d: usize,
        lr: [(usize, usize); d]
    }

    let mut from_left = vec![0; n];
    from_left[0] = a[0];
    for i in 1..n {
        from_left[i] = a[i].max(from_left[i - 1])
    }

    let mut from_right = vec![0; n];
    from_right[0] = a[n - 1];
    for i in 1..n {
        from_right[i] = a[n - i - 1].max(from_right[i - 1])
    }

    for i in 0..d {
        let l = lr[i].0 - 2;
        let r = n - lr[i].1 - 1;

        // println!("l: {}, r: {}", l, r);
        // println!("from_left: {:?}, from_right: {:?}", from_left, from_right);

        println!("{}", from_left[l].max(from_right[r]))
    }
}

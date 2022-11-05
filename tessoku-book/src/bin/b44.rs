use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n],
        q: usize,
        queries: [(Usize1, Usize1, Usize1); q]
    }

    let mut order = vec![0; n];
    for i in 0..n {
        order[i] = i;
    }

    for i in 0..q {
        let cmd = queries[i].0;
        let left = queries[i].1;
        let right = queries[i].2;

        if cmd == 0 {
            order.swap(left, right)
        } else {
            let x = order[left];
            let y = right;

            println!("{}", a[x][y])
        }
    }
}

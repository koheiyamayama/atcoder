use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut t: isize,
        a: [isize; n-1],
        xy: [(isize, isize); m]
    }

    let mut bonus = vec![0; n];
    for i in 0..m {
        bonus[xy[i].0 as usize - 1] = xy[i].1;
    }

    let mut flag = true;
    for i in 0..n - 1 {
        if t - a[i] > 0 {
            t = t - a[i] + bonus[i + 1];
        } else {
            flag = false;
            break;
        }
    }

    if flag {
        println!("Yes")
    } else {
        println!("No")
    }
}

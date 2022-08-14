use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        t: [(usize, usize); m]
    }

    let mut a: Vec<bool> = vec![false; n];
    let mut b: Vec<bool> = vec![false; n];

    for i in 0..m {
        if t[i].0 == 1 {
            a[t[i].1] = true;
        }

        if t[i].1 == n {
            b[t[i].0] = true;
        }
    }

    let mut flag = false;
    for i in 0..n {
        if a[i] && b[i] {
            flag = true;
            break;
        }
    }

    if flag {
        println!("POSSIBLE")
    } else {
        println!("IMPOSSIBLE")
    }
}

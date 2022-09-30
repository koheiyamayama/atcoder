use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
        lr: [(usize, usize); q]
    }

    let mut acum = vec![0; n + 1];
    for i in 1..n + 1 {
        acum[i] = a[i - 1] + acum[i - 1];
    }

    for i in 0..q {
        let l = lr[i].0;
        let r = lr[i].1;
        let atari = acum[r] - acum[l - 1];
        let hazure = (r - l + 1) - atari;

        if atari > hazure {
            println!("win")
        } else if atari < hazure {
            println!("lose")
        } else {
            println!("draw")
        }
    }
}

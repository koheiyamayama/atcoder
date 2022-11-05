use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        ab: [(usize, char); n]
    }

    let mut ans = 0;
    for i in 0..n {
        if ab[i].1 == 'W' {
            ans = ans.max(ab[i].0)
        } else {
            ans = ans.max(l - ab[i].0)
        }
    }

    println!("{}", ans)
}

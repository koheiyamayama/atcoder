use proconio::input;

fn main() {
    input! {
        n: usize,
        mut d: [usize; n]
    }

    d.sort();
    let a = n / 2 - 1;
    let b = n / 2;

    let c = d[a];
    let e = d[b];

    println!("{}", e - c)
}

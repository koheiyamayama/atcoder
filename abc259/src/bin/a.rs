use proconio::input;

fn main() {
    input! {
        i: [usize; 5]
    }

    let n = i[0];
    let m = i[1];
    let x = i[2];
    let t = i[3];
    let d = i[4];

    let age_zero = t - x * d;

    if x <= m {
        println!("{}", t)
    } else {
        println!("{}", age_zero + m * d)
    }
}

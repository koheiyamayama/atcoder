use proconio::input;

fn main() {
    input! {
        n: usize,
        mut m: isize,
        mut ab: [(isize, isize); n]
    }

    ab.sort_by(|a, b| a.0.cmp(&b.0));

    let mut sum = 0;
    for i in 0..n {
        if m == 0 {
            break;
        }

        if m - ab[i].1 >= 0 {
            sum += ab[i].0 * ab[i].1;
            m -= ab[i].1;
        } else {
            sum += ab[i].0 * m;
            m = 0;
        }
    }

    println!("{}", sum)
}

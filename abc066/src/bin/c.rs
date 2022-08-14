use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut r: Vec<usize> = Vec::new();
    let len = a.len();
    let len_minus_one = len - 1;
    let half = a.len() / 2;
    for i in 0..n {
        if i == half {
            r.push(0)
        } else if half > i {
            // 前半部分
            r.push(len_minus_one - i * 2)
        } else {
            // 後半部分
            r.push(i * 2 - len)
        }
    }

    for (index, i) in r.iter().enumerate() {
        if index == len_minus_one {
            println!("{}", a[*i])
        } else {
            print!("{} ", a[*i])
        }
    }
}

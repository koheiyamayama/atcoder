use proconio::input;

fn main() {
    input! {
        n: usize,
        l: [usize; n]
    }

    let mut count = 0;
    for i in 0..n {
        for j in i..n {
            for k in j..n {
                if l[i] == l[j]
                    || l[i] == l[k]
                    || l[j] == l[k]
                    || !(l[i] + l[j] > l[k])
                    || !(l[i] + l[k] > l[j])
                    || !(l[j] + l[k] > l[i])
                {
                    continue;
                } else {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count)
}

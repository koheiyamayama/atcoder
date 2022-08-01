use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut count: usize = 0;
    for i in 0..n {
        for j in (i + 1)..n {
            // println!("a[i]: {}", a[i]);
            // println!("a[j]: {}", a[j]);
            // println!("i: {}", i + 1);
            // println!("j: {}", j + 1);

            if i < j && min(a[i], a[j]) == i + 1 && max(a[i], a[j]) == j + 1 {
                count += 1;
            }
        }
    }

    println!("{}", count)
}

fn max(n: usize, m: usize) -> usize {
    if n >= m {
        return n;
    } else {
        return m;
    }
}

fn min(n: usize, m: usize) -> usize {
    if n <= m {
        return n;
    } else {
        return m;
    }
}

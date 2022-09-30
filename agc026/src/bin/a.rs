use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut count = 0;
    let mut kukan: Vec<Vec<usize>> = vec![vec![a[0]]];
    for i in 1..n {
        let last = kukan.last().unwrap().last();
        if last.is_some() && *last.unwrap() == a[i] {
            kukan.last_mut().unwrap().push(a[i]);
        } else if last.is_some() {
            kukan.push(vec![a[i]])
        } else {
        }
    }

    for i in 0..kukan.len() {
        count += kukan[i].len() / 2
    }

    println!("{}", count)
}

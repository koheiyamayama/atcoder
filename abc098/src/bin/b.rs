use proconio::input;

fn main() {
    input! {
        n: i64,
        s: String
    }

    let mut results = vec!();
    for i in 0..n {
        let tmp_c = s.chars().collect::<Vec<_>>();
        let (c1, c2) = tmp_c.split_at(i as usize);
        let mut c1 = c1.to_vec();
        c1.sort();
        c1.dedup();
        let mut c2 = c2.to_vec();
        c2.sort();
        c2.dedup();
        let mut count = 0;
        for c in c1.iter() {
            if c2.contains(c) {
                count += 1;
            }
        }
        results.push(count)
    }

    println!("{}", results.iter().max().unwrap())
}

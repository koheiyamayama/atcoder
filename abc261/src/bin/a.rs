use proconio::input;

fn main() {
    input! {
        l1: usize,
        r1: usize,
        l2: usize,
        r2: usize
    }

    let mut r: Vec<usize> = Vec::new();
    let mut l: Vec<usize> = Vec::new();

    for red in l1..=r1 {
        r.push(red);
    }

    for blue in l2..=r2 {
        l.push(blue);
    }


    let mut results = -1;

    if r.len() <= l.len() {
        for i in l {
            if r.contains(&i) {
                results += 1;
            }
        }
    } else {
        for i in r {
            if l.contains(&i) {
                results += 1;
            }
        }        
    }

    if results == -1 {
        println!("{}", 0)
    } else {
        println!("{}", results)
    }
}

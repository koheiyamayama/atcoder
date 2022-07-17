use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        z: usize,
        mut math: [usize; n],
        mut english: [usize; n]
    }

    let mut results: Vec<usize> = Vec::new();

    //              index, math,  eng,   total
    let mut t: Vec<(usize, usize, usize, usize)> = Vec::new();
    for i in 0..n {
        t.push(
            (i+1, math[i], english[i], math[i] + english[i])
        )
    }


    // math
    t.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    t.sort_by(|tuple_one, tuple_two| tuple_two.1.partial_cmp(&tuple_one.1).unwrap());
    for _tm in 0..x {
        results.push(t[0].0);
        t.remove(0);
    }


    // eng
    t.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    t.sort_by(|tuple_one, tuple_two| tuple_two.2.partial_cmp(&tuple_one.2).unwrap());
    for _te in 0..y {
        results.push(t[0].0);
        t.remove(0);
    }

    // sum
    t.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    t.sort_by(|tuple_one, tuple_two| tuple_two.3.partial_cmp(&tuple_one.3).unwrap());
    for _ts in 0..z {
        results.push(t[0].0);
        t.remove(0);
    }

    results.sort();
    for r in results {
        println!("{}", r)
    }
}

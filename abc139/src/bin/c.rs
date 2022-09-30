use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [usize; n]
    }

    let mut vec: Vec<Vec<usize>> = Vec::new();
    vec.push(vec![h[0]]);
    for i in 1..n {
        if h[i] <= *vec.last().unwrap().last().unwrap() {
            vec.last_mut().unwrap().push(h[i])
        } else {
            vec.push(vec![h[i]])
        }
    }

    let mut max = std::usize::MIN;
    for i in 0..vec.len() {
        let move_count = vec[i].len() - 1;
        max = max.max(move_count)
    }

    println!("{}", max)
}

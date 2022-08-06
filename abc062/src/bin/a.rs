use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize
    }

    let a = vec![1, 3, 5, 7, 8, 10, 12];
    let b = vec![4, 6, 9, 11];
    let c = vec![2];

    if (a.contains(&x) && a.contains(&y))
        || (b.contains(&x) && b.contains(&y))
        || (c.contains(&x) && c.contains(&y))
    {
        println!("Yes")
    } else {
        println!("No")
    }
}

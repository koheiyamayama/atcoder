use proconio::input;

fn main() {
    input! {
        n: i32,
        d: [i32; n]
    }

    let mut mochi: Vec<i32> = vec!();

    for number in d {
        if mochi.contains(&number) {
            continue;
        }
        mochi.push(number)
    }

    println!("{}", mochi.len());
}

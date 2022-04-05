use proconio::input;

fn main() {
    input! {
        n: i32,
        mut a: [i32; n]
    }

    let mut alice: Vec<i32> = vec!();
    let mut bob: Vec<i32> = vec!();
    a.sort_by(|b, c| c.cmp(b) );

    let mut i = 1;
    for number in a {
        if i % 2 == 0 {
            bob.push(number);
        } else {
            alice.push(number);
        }
        i += 1;
    }

    println!("{}", alice.iter().sum::<i32>() - bob.iter().sum::<i32>());
}

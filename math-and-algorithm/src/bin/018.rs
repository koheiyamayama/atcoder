use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut hundred: u64 = 0;
    let mut two_hundred = 0;
    let mut three_hundred = 0;
    let mut four_hundred = 0;

    for i in a {
        if i == 100 {
            hundred += 1;
        } else if i == 200 {
            two_hundred += 1;
        } else if i == 300 {
            three_hundred += 1;
        } else if i == 400 {
            four_hundred += 1;
        }
    }

    println!("{}", hundred * four_hundred + two_hundred * three_hundred)
}

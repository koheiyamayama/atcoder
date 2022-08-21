use proconio::input;

fn main() {
    input! {
        n: usize
    }

    let mut day = 1;
    let mut flag = true;
    let mut sum = 0;
    while flag {
        sum += day;
        if sum >= n {
            flag = false;
        } else {
            day += 1;
        }
    }

    println!("{}", day)
}

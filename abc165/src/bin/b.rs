use proconio::input;

fn main() {
    input! {
        x: usize
    }

    let mut flag = true;
    let mut sum = 100;
    let mut day = 0;
    while flag {
        day += 1;
        sum = sum + sum / 100;
        if x <= sum {
            flag = false;
        }
    }

    println!("{}", day)
}

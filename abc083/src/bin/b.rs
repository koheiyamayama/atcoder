use proconio::input;

fn digit_sum(num: i32) -> i32 {
    num
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap() as i32)
        .collect::<Vec<_>>()
        .iter()
        .sum()
}

fn main() {
    input! {
        n: i32,
        a: i32,
        b: i32
    }

    let mut nums: Vec<i32> = vec!();
    for num in 0..=n {
        let digit_sum = digit_sum(num);
        if digit_sum >= a && digit_sum <= b {
            nums.push(num)
        };
    }

    println!("{}", nums.iter().sum::<i32>())
}

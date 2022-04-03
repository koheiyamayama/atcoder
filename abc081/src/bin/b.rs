use proconio::input;

fn all_even (nums: &Vec<i32>) -> bool {
    let mut result = true;
    for num in nums {
        if num % 2 != 0 {
            result = false;
            break;
        }
    }
    return result
}

fn main() {
    input! {
        n: i32,
        mut nums: [i32; n]
    }

    let mut i = 0;

    loop {
        if !all_even(&nums) {
            break;
        }
        nums = nums.iter().map(|num| {num / 2}).collect();
        i += 1;
    };

    println!("{}", i);
}

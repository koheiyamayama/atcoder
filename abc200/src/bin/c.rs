use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut nums: Vec<isize> = vec![0; 200];
    for i in 0..n {
        nums[a[i] % 200] += 1;
    }

    let mut count = 0;
    for i in 0..200 {
        count += nums[i] * (nums[i] - 1) / 2
    }

    println!("{}", count)
}

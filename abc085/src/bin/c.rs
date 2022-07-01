use proconio::input;

fn main() {
    input! {
        n: i64,
        y: i64
    }

    let mut counter: Vec<i64>;

    counter = vec!(-1, -1, -1);

    for ten_thousand_count in 0..=n {
        let count_one = n - ten_thousand_count;
        for five_thousand_count in 0..=count_one {
            let thousand_count = count_one - five_thousand_count;
            if ten_thousand_count * 10_000 + five_thousand_count * 5_000 + thousand_count * 1_000 == y {
                counter = vec!(ten_thousand_count, five_thousand_count, thousand_count);
            }
        }
    }

    println!("{} {} {}", counter[0], counter[1], counter[2]);
}

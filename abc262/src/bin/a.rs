use proconio::input;

fn main() {
    input! {
        mut y: usize
    }

    let mut result = 0;
    for i in 0..5 {
        if i == 0 {
        } else {
            y += 1;
        }
        if y % 4 == 2 {
            result = y;
            break;
        }
    }

    println!("{:?}", result)
}

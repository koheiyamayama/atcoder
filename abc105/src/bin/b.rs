use proconio::input;

fn main() {
    input! {
        n: usize
    }

    for i in 0..30 {
        for j in 0..30 {
            if i * 4 + j * 7 == n {
                println!("Yes");
                return;
            }
        }
    }

    println!("No")
}

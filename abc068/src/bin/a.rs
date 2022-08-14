use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars
    }

    print!("ABC");
    for nn in n {
        print!("{}", nn);
    }
    println!("")
}

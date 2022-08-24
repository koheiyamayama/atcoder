use proconio::input;

fn main() {
    input! {
        s: String
    }

    if String::from("Hello,World!") == s {
        println!("AC")
    } else {
        println!("WA")
    }
}

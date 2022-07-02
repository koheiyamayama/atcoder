use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize
    }

    let strings = String::from("abcdefghijklmnopqrstuvwxyz").to_uppercase();
    let mut new_strings = String::new();

    for s in strings.split("") {
        let s = &s.repeat(n);
        new_strings.push_str(s)
    }

    match new_strings.chars().nth(x-1) {
        Some(r) => println!("{}", r),
        None => println!()
    }
}

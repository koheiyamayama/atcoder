use proconio::input;

fn main() {
    input! {
        h: u8,
        w: u8,
        strings: [String; h]
    }

    for s in strings.iter() {
        println!("{}", s);
        println!("{}", s);
    }
}

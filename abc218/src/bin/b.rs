use proconio::input;

fn main() {
    input! {
        p: [u8; 26]
    }

    for i in 0..p.len() {
        print!("{}", (p[i] + 96) as char)
    }
    println!("")
}

// charとu8の対応
// 'a'が97
// 'z'が122
// 'A'が65
// 'Z'が90

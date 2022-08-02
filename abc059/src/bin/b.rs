use proconio::input;

fn main() {
    input! {
        mut a: String,
        mut b: String
    }

    // くそ無駄に長いコードを書いてしまった。
    // let mut aa: Vec<usize> = vec![0; 101];
    // let a: Vec<char> = a.chars().collect();

    // for i in 0..a.len() {
    //     aa[100 - i] = a[a.len() - 1 - i].to_digit(10).unwrap() as usize;
    // }

    // let mut bb: Vec<usize> = vec![0; 101];
    // let b: Vec<char> = b.chars().collect();

    // for i in 0..b.len() {
    //     bb[100 - i] = b[b.len() - 1 - i].to_digit(10).unwrap() as usize;
    // }

    // let mut r = String::new();
    // for i in 0..101 {
    //     if aa[i] as u8 == bb[i] as u8 {
    //         r = String::from("EQUAL");
    //     } else if aa[i] as u8 > bb[i] as u8 {
    //         r = String::from("GREATER");
    //         break;
    //     } else {
    //         r = String::from("LESS");
    //         break;
    //     }
    // }

    if a.len() > b.len() {
        println!("GREATER")
    } else if a.len() < b.len() {
        println!("LESS")
    } else {
        if a > b {
            println!("GREATER")
        } else if a < b {
            println!("LESS")
        } else {
            println!("EQUAL")
        }
    }
}

use proconio::input;

fn main() {
    input! {
        a: u8,
        b: u8,
        c: u8
    }

    let ary = [a, b, c];
    let mut s = 0;
    let mut f = 0;
    for i in ary.iter() {
        if *i == 7 {
            s += 1;
        } else if *i == 5 {
            f += 1;
        }
    }

    if s == 1 && f == 2 {
        println!("YES")
    } else {
        println!("NO")
    }
}

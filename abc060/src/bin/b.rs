use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize
    }

    let mut flag = false;
    for i in 1..=b {
        if i * a % b == c {
            flag = true
        }
    }

    if flag {
        println!("YES")
    } else {
        println!("NO")
    }
}

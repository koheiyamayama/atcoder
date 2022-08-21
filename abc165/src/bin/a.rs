use proconio::input;

fn main() {
    input! {
        k: usize,
        a: usize,
        b: usize
    }

    let mut flag = false;
    for i in a..=b {
        if i % k == 0 {
            flag = true;
            break;
        }
    }

    if flag {
        println!("OK")
    } else {
        println!("NG")
    }
}

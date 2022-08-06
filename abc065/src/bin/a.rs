use proconio::input;

fn main() {
    input! {
        x: isize,
        a: isize,
        b: isize
    }

    let shoumikigen = 0;
    let buy_at = shoumikigen - a;
    let safe_at = x;
    let eat_at = b - a;

    if buy_at <= eat_at && eat_at <= shoumikigen {
        println!("delicious")
    } else if shoumikigen < eat_at && eat_at <= safe_at {
        println!("safe")
    } else {
        println!("dangerous")
    }
}

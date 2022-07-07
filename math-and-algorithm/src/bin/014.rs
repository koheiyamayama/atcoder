use proconio::input;

fn main() {
    input! {
        n: f64
    }

    
    let s = n.sqrt().floor() as i64;
    let mut n = n as i64;
    let mut vec = vec!();

    for i in 2..=s {
        while n % i == 0 {
            vec.push(i.to_string());
            n /= i;
        }
    }

    if s <= n {
        vec.push(n.to_string());
    }

    println!("{}", vec.join(" "));
}

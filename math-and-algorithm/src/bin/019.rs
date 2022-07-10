use proconio::input;

fn main() {
    input! {
        n: u64,
        a: [u64; n]
    }

    let mut red: u64 = 0;
    let mut yellow: u64 = 0;
    let mut blue: u64 = 0;

    for i in a {
        if i == 1 {
            red += 1;            
        } else if i == 2 {
            yellow += 1;
        } else if i == 3 {
            blue += 1;
        }
    }

    let red = combination_two(red);
    let yellow = combination_two(yellow);
    let blue = combination_two(blue);

    println!("{}", red + yellow + blue);
}

fn combination_two(n: u64) -> u64 { 
    return n * (n - 1) / 2
 }

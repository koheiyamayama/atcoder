use proconio::input;

fn main() {
    input! {
        mut n: usize
    }

    let mut binary: Vec<usize> = vec![0; 10];

    let mut index = 10;
    while n != 0 {
        index -= 1;

        binary[index] = n % 2;
        n = n / 2;
    }

    for i in 0..binary.len() {
        print!("{}", binary[i])
    }
    println!("")
}

use proconio::input;

fn main() {
    input! {
        n: usize,
        operations: [(char, isize); n]
    }

    let mut sum = 0;

    for i in 0..n {
        let calc = operations[i].0;
        let num = operations[i].1;
        if calc == '+' {
            sum += num;
            sum = mod_minus(sum, 10_000);
        } else if calc == '-' {
            sum -= num;
            sum = mod_minus(sum, 10_000);
        } else if calc == '*' {
            sum *= num;
            sum = mod_minus(sum, 10_000);
        }

        println!("{}", sum)
    }
}

fn mod_minus(val: isize, m: isize) -> isize {
    let mut res = val % m;
    if res < 0 {
        res += m;
    }
    return res;
}

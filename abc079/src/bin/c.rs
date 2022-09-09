use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut abcd: Chars
    }

    let op = vec!['+', '-'];
    let abcd: Vec<isize> = abcd.iter().map(|e| *e as isize - 48).collect();
    let mut sum = 0;

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                if op[i] == '+' {
                    sum = abcd[0] + abcd[1];
                } else {
                    sum = abcd[0] - abcd[1];
                }
                if op[j] == '+' {
                    sum += abcd[2];
                } else {
                    sum -= abcd[2];
                }
                if op[k] == '+' {
                    sum += abcd[3];
                } else {
                    sum -= abcd[3];
                }

                if sum == 7 {
                    println!(
                        "{}{}{}{}{}{}{}={}",
                        abcd[0], op[i], abcd[1], op[j], abcd[2], op[k], abcd[3], sum
                    );
                    return;
                }
                sum = 0;
            }
        }
    }
}

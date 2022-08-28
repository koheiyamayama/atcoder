use proconio::input;

fn main() {
    input! {
        a: [[usize; 3]; 3],
        n: usize,
        b: [usize; n]
    }

    let mut bingo = [[false; 3]; 3];
    for i in 0..n {
        for j in 0..3 {
            for k in 0..3 {
                if a[j][k] == b[i] {
                    bingo[j][k] = a[j][k] == b[i];
                    break;
                }
            }
        }
    }

    if is_bingo(bingo) {
        println!("Yes")
    } else {
        println!("No")
    }
}

fn is_bingo(b: [[bool; 3]; 3]) -> bool {
    return (b[0][0] && b[0][1] && b[0][2])
        || (b[1][0] && b[1][1] && b[1][2])
        || (b[2][0] && b[2][1] && b[2][2])
        || (b[0][0] && b[1][0] && b[2][0])
        || (b[0][1] && b[1][1] && b[2][1])
        || (b[0][2] && b[1][2] && b[2][2])
        || (b[0][0] && b[1][1] && b[2][2])
        || (b[2][0] && b[1][1] && b[0][2]);
}

use proconio::input;

fn main() {
    input! {
        c: [[isize; 3]; 3]
    }

    for i in 0..=100 {
        let a1 = i;
        let b1 = c[0][0] - a1;
        let a2 = c[1][0] - b1;
        let b2 = c[0][1] - a1;
        let a3 = c[2][0] - b1;
        let b3 = c[2][2] - a3;

        if c[0][0] == a1 + b1
            && c[0][1] == a1 + b2
            && c[0][2] == a1 + b3
            && c[1][0] == a2 + b1
            && c[1][1] == a2 + b2
            && c[1][2] == a2 + b3
            && c[2][0] == a3 + b1
            && c[2][1] == a3 + b2
            && c[2][2] == a3 + b3
        {
            println!("Yes");
            return;
        }
    }

    println!("No")
}

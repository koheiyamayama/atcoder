use proconio::input;

fn main() {
    input! {
        mut x: usize,
        mut y: usize
    }

    if x == 1 && y == 1 {
        println!("0");
        return;
    }

    let mut ans: Vec<(usize, usize)> = Vec::new();
    ans.push((x, y));

    loop {
        if x >= y {
            x = x - y;
        } else {
            y = y - x;
        }

        if x == 1 && y == 1 {
            break;
        } else {
            ans.push((x, y));
        }
    }

    ans.reverse();
    println!("{}", ans.len());
    for (i, j) in ans {
        println!("{} {}", i, j)
    }
}

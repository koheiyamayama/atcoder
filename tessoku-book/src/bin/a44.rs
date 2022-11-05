use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut queries: Vec<(usize, usize, usize)> = Vec::new();
    for i in 0..q {
        input! {command: usize}
        if command == 1 {
            input! {x: Usize1, y: usize}
            queries.push((command, x, y))
        } else if command == 2 {
            queries.push((command, 0, 0))
        } else {
            input! {x: Usize1}
            queries.push((command, x, 0))
        }
    }

    let mut ans = vec![1; n];
    for i in 1..=n {
        ans[i - 1] = i;
    }

    let mut reversed = false;
    for q in queries {
        if q.0 == 1 {
            if reversed {
                ans[n - q.1 - 1] = q.2;
            } else {
                ans[q.1] = q.2;
            }
        } else if q.0 == 2 {
            reversed = !reversed;
        } else if q.0 == 3 {
            if reversed {
                println!("{}", ans[n - q.1 - 1])
            } else {
                println!("{}", ans[q.1])
            }
        }
        // println!("{:?}", ans)
    }
}

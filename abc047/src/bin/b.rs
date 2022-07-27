// これを移した
// https://onecheese.com/2021/11/14/atcoder-abc-047-b/

use proconio::input;

fn main() {
    input! {
        w: isize,
        h: isize,
        n: isize,
        r: [(isize, isize, isize); n]
    }

    let mut x_1 = 0;
    let mut x_2 = w;
    let mut y_1 = 0;
    let mut y_2 = h;

    for t in r {
        if t.2 == 1 {
            // x=0からt.0まで塗りつぶしてることを管理する
            x_1 = max(x_1, t.0)
        } else if t.2 == 2 {
            // t.0からwまで塗りつぶしていることを管理する
            x_2 = min(x_2, t.0)
        } else if t.2 == 3 {
            // y=0からt.1まで塗りつぶしていることを管理する
            y_1 = max(t.1, y_1)
        } else if t.2 == 4 {
            // t.1からhまで塗りつぶしていることを管理する
            y_2 = min(t.1, y_2)
        }
    }

    
    let x_diff = max(0, x_2 - x_1);
    let y_diff = max(0, y_2 - y_1);

    println!("{}", x_diff*y_diff)

}

fn max(n: isize, m: isize) -> isize {
    if n >= m {
        return n
    } else {
        return m
    }
}

fn min(n: isize, m: isize) -> isize {
    if n <= m {
        return n
    } else {
        return m
    }
}

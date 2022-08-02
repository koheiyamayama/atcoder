use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        o: Chars,
        e: Chars
    }

    let mut ans = vec!['0'; o.len() + e.len()];

    let mut index = 0;
    for m in o {
        if index % 2 == 0 {
            ans[index] = m;
        }

        index += 2;
    }

    index = 1;
    for n in e {
        if index % 2 != 0 {
            ans[index] = n;
        }

        index += 2;
    }

    for a in ans {
        print!("{}", a)
    }
    println!("")
}

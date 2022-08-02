// https://scrapbox.io/yu2ta7ka/AtCoder_ABC_058_C_-_%E6%80%AA%E6%96%87%E6%9B%B8_(300_%E7%82%B9)
// これを参考にした

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n]
    }

    let mut letters = vec![vec![0; 27]; n];

    let mut index = 0;
    for chars in s {
        for c in chars {
            letters[index][c as usize - 'a' as usize] += 1;
        }
        index += 1;
    }

    let mut result_letters = vec![std::u8::MAX; 27];
    for m in 0..n {
        for i in 0..27 {
            result_letters[i] = min(letters[m][i], result_letters[i]);
        }
    }

    let mut flag = false;
    for i in 0..27 {
        if result_letters[i] > 0 {
            flag = true;
            print!(
                "{}",
                (((i as u8) + ('a' as u8)) as char)
                    .to_string()
                    .repeat(result_letters[i] as usize)
            )
        }
    }

    if flag {
        println!("")
    }
}

fn min(n: u8, m: u8) -> u8 {
    if n <= m {
        return n;
    } else {
        return m;
    }
}

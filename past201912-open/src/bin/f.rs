use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }

    let mut words: Vec<String> = Vec::new();
    let mut word: Vec<char> = Vec::new();
    let mut flag = false;

    for i in 0..s.len() {
        if s[i].is_uppercase() {
            flag = !flag;
        }
        if flag {
            word.push(s[i])
        } else {
            word.push(s[i]);
            words.push(word.clone().into_iter().collect::<String>());
            word.clear();
        }
    }

    words.sort_by(|a, b| a.to_lowercase().partial_cmp(&b.to_lowercase()).unwrap());
    for i in 0..words.len() {
        print!("{}", words[i])
    }
    println!("")
}

use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        strings: [String; n]
    }

    let mut results: HashMap<String, usize> = HashMap::new();

    // TLEだったコード
    // for string in strings {
    //     match results.clone().get(&string) {
    //         Some(s) => {
    //             results.insert(string.to_owned(), *s+1);
    //             println!("{}({})", string, *s);
    //         },
    //         None => { 
    //             results.insert(string.to_owned(), 1);
    //             println!("{}", string);
    //          }
    //     };
    // }

    let mut c: usize = 0;
    for string in strings {
        match results.get_mut(&string) {
            Some(v) => {
                c = *v;
                println!("{}({})", string, *v);
            }
            None => {
                c = 0;
                println!("{}", string);
            }
        }

        results.insert(string, c+1);
    }
}

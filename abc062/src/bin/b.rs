use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        mut a: [Chars; h]
    }

    a.insert(0, vec!['#'; w + 2]);
    a.push(vec!['#'; w + 2]);

    let len = a.len();

    for (outer_index, outer) in a.iter().enumerate() {
        for (inner_index, inner) in outer.iter().enumerate() {
            if outer_index == 0 || outer_index == len - 1 {
                print!("{}", inner);
            } else {
                if inner_index == 0 && inner_index == w - 1 {
                    print!("#{}#", inner)
                } else if inner_index == 0 {
                    print!("#{}", inner)
                } else if inner_index == w - 1 {
                    print!("{}#", inner)
                } else {
                    print!("{}", inner)
                }
            }
        }
        println!("")
    }
}

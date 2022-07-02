use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut s: String,
        queries: [(usize, usize); q]
    };

    // println!("{}", n);
    // println!("{}", q);
    // println!("{}", s);
    // println!("{:?}", queries);

    let mut p = 0;
    for query in queries {
        if query.0 == 1 {
            p += query.1;
        } else if query.0 == 2 {
            
        }
    }
}

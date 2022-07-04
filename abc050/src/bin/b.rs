use proconio::input;

fn main() {
    input! {
        n: usize,
        t: [usize; n],
        m: usize,
        p: [(usize, usize); m]
    };

    for o in &p {
        let mut index = 0;
        let mut total = 0;
        for i in &t {
            if o.0 == index+1 {
                total += o.1
            } else {
                total += i
            }
            index += 1;
        }
        println!("{}", total)
    }
}

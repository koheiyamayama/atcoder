use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        w: usize
    }

    let not_ans = "UNSATISFIABLE";

    let kg = w * 1000;
    let mut l = std::usize::MAX;
    let mut r = std::usize::MIN;

    for n in 1..=kg {
        if a * n <= kg && kg <= b * n {
            l = l.min(n);
            r = r.max(n);
        }
    }

    if l == std::usize::MAX && r == std::usize::MIN {
        println!("{}", not_ans)
    } else {
        println!("{} {}", l, r)
    }
}

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut py: [(usize, usize); m]
    }

    let mut i_py = vec![(0, 0, 0); m];
    for i in 0..m {
        i_py[i] = (i, py[i].0, py[i].1);
    }

    i_py.sort_by(|a, b| match a.1.cmp(&b.1) {
        std::cmp::Ordering::Equal => a.2.cmp(&b.2),
        other => other,
    });
    let mut indexed_py = vec![(0, 0, 0, 0); m];
    let mut grouped_by_ranking = 1;
    for i in 0..m {
        if i > 0 && i_py[i - 1].1 != i_py[i].1 {
            grouped_by_ranking = 1;
        }
        let tuple = (i_py[i].0, grouped_by_ranking, i_py[i].1, i_py[i].2);
        grouped_by_ranking += 1;
        indexed_py[i] = tuple;
    }

    // index, ranking, prefecture, year
    indexed_py.sort_by(|a, b| a.0.cmp(&b.0));

    for i in 0..m {
        println!("{:06}{:06}", indexed_py[i].2, indexed_py[i].1)
    }
}

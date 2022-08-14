use proconio::input;

fn main() {
    input! {
        r: usize,
        c: usize
    };

    let grid = vec![
        vec![
            "black", "black", "black", "black", "black", "black", "black", "black", "black",
            "black", "black", "black", "black", "black", "black",
        ],
        vec![
            "black", "white", "white", "white", "white", "white", "white", "white", "white",
            "white", "white", "white", "white", "white", "black",
        ],
        vec![
            "black", "white", "black", "black", "black", "black", "black", "black", "black",
            "black", "black", "black", "black", "white", "black",
        ],
        vec![
            "black", "white", "black", "white", "white", "white", "white", "white", "white",
            "white", "white", "white", "black", "white", "black",
        ],
        vec![
            "black", "white", "black", "white", "black", "black", "black", "black", "black",
            "black", "black", "white", "black", "white", "black",
        ],
        vec![
            "black", "white", "black", "white", "black", "white", "white", "white", "white",
            "white", "black", "white", "black", "white", "black",
        ],
        vec![
            "black", "white", "black", "white", "black", "white", "black", "black", "black",
            "white", "black", "white", "black", "white", "black",
        ],
        vec![
            "black", "white", "black", "white", "black", "white", "black", "white", "black",
            "white", "black", "white", "black", "white", "black",
        ],
        vec![
            "black", "white", "black", "white", "black", "white", "black", "black", "black",
            "white", "black", "white", "black", "white", "black",
        ],
        vec![
            "black", "white", "black", "white", "black", "white", "white", "white", "white",
            "white", "black", "white", "black", "white", "black",
        ],
        vec![
            "black", "white", "black", "white", "black", "black", "black", "black", "black",
            "black", "black", "white", "black", "white", "black",
        ],
        vec![
            "black", "white", "black", "white", "white", "white", "white", "white", "white",
            "white", "white", "white", "black", "white", "black",
        ],
        vec![
            "black", "white", "black", "black", "black", "black", "black", "black", "black",
            "black", "black", "black", "black", "white", "black",
        ],
        vec![
            "black", "white", "white", "white", "white", "white", "white", "white", "white",
            "white", "white", "white", "white", "white", "black",
        ],
        vec![
            "black", "black", "black", "black", "black", "black", "black", "black", "black",
            "black", "black", "black", "black", "black", "black",
        ],
    ];

    println!("{}", grid[r - 1][c - 1])
}

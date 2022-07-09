use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
        d: f64
    }

    // println!("{}, {}, {}", a, b, d);


    let d = d.to_radians();
    
    let x = a * d.cos() -  b * d.sin();
    let y = b * d.cos() + a * d.sin();

    println!("{:.20} {:.20}", x, y)
}

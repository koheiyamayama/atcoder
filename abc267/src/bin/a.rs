use proconio::input;

fn main() {
    input! {
        s: String
    }

    let days: Vec<String> = vec![
        String::from("Monday"),
        String::from("Tuesday"),
        String::from("Wednesday"),
        String::from("Thursday"),
        String::from("Friday"),
    ];

    if s == days[0] {
        println!("5")
    } else if s == days[1] {
        println!("4")
    } else if s == days[2] {
        println!("3")
    } else if s == days[3] {
        println!("2")
    } else if s == days[4] {
        println!("1")
    }
}

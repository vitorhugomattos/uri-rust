use std::io;

fn main() {
    let mut p1 = String::new();
    let mut p2 = String::new();

    io::stdin().read_line(&mut p1).unwrap();
    io::stdin().read_line(&mut p2).unwrap();

    let p1 = p1
        .split_ascii_whitespace()
        .map(|f| f.parse::<f64>().unwrap())
        .collect::<Vec<_>>();

    let p2 = p2
        .split_ascii_whitespace()
        .map(|f| f.parse::<f64>().unwrap())
        .collect::<Vec<_>>();
    
    println!("{:.4}", ((p2.get(0).unwrap() - p1.get(0).unwrap()).powi(2) + (p2.get(1).unwrap() - p1.get(1).unwrap()).powi(2)).sqrt())
}

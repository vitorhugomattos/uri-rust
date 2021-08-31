use std::io;

fn main() {
    let mut x = String::new();
    let mut y = String::new();

    io::stdin().read_line(&mut x).unwrap();
    io::stdin().read_line(&mut y).unwrap();

    let x = x.trim().parse::<u32>().unwrap();
    let y = y.trim().parse::<f64>().unwrap();

    println!("{:.3} km/l", x as f64 / y)
}

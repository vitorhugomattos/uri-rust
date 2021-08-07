use std::io;

fn main() {
    let mut a = String::new();
    let mut b = String::new();

    io::stdin().read_line(&mut a).unwrap();
    io::stdin().read_line(&mut b).unwrap();

    let a = a.trim().parse::<f64>().unwrap();
    let b = b.trim().parse::<f64>().unwrap();

    println!("MEDIA = {:.5}", (a * 3.5 + b * 7.5) / 11f64);
}

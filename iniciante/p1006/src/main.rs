use std::io;

fn main() {
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

    io::stdin().read_line(&mut a).unwrap();
    io::stdin().read_line(&mut b).unwrap();
    io::stdin().read_line(&mut c).unwrap();

    let a = a.trim().parse::<f64>().unwrap();
    let b = b.trim().parse::<f64>().unwrap();
    let c = c.trim().parse::<f64>().unwrap();

    println!("MEDIA = {:.1}", (a * 2f64 + b * 3f64 + c * 5f64) / 10f64);
}

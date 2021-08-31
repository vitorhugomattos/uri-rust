use std::io;

fn main() {
    let mut raio = String::new();

    io::stdin().read_line(&mut raio).unwrap();

    let raio = raio.trim().parse::<f64>().unwrap();

    println!("VOLUME = {:.3}", 4f64/3f64 * 3.14159 * raio.powi(3))
}

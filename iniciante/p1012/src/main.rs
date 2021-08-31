use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let numbers = line
        .split_ascii_whitespace()
        .map(|f| f.parse::<f64>().unwrap())
        .collect::<Vec<_>>();

    let a = numbers.get(0).unwrap();
    let b = numbers.get(1).unwrap();
    let c = numbers.get(2).unwrap();

    println!("TRIANGULO: {:.3}", (a * c) / 2f64);
    println!("CIRCULO: {:.3}", 3.14159 * c.powi(2));
    println!("TRAPEZIO: {:.3}", ((a + b) * c) / 2f64);
    println!("QUADRADO: {:.3}", b.powi(2));
    println!("RETANGULO: {:.3}", a * b);
}

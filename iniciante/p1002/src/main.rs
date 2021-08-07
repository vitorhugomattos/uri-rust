fn main() {
    let mut raio = String::new();

    std::io::stdin().read_line(&mut raio).unwrap();

    let raio = raio.trim().parse::<f64>().unwrap();

    println!("A={:.4}", 3.14159 * raio.powi(2));
}

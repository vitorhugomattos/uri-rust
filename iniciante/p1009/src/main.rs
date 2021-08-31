use std::io;

fn main() {
    let mut nome = String::new();
    let mut salario = String::new();
    let mut vendas = String::new();

    io::stdin().read_line(&mut nome).unwrap();
    io::stdin().read_line(&mut salario).unwrap();
    io::stdin().read_line(&mut vendas).unwrap();

    let salario = salario.trim().parse::<f64>().unwrap();
    let vendas = vendas.trim().parse::<f64>().unwrap();

    println!("TOTAL = R$ {:.2}", salario + (vendas * 0.15));
}

use std::io;

fn main() {
    let mut peca1 = String::new();
    let mut peca2 = String::new();

    io::stdin().read_line(&mut peca1).unwrap();
    io::stdin().read_line(&mut peca2).unwrap();

    let peca1 = peca1
        .split_ascii_whitespace()
        .enumerate()
        .filter(|(i, _)| *i >= 1)
        .map(|(_, a)| a.parse::<f64>().unwrap())
        .collect::<Vec<f64>>();

    let peca2 = peca2
        .split_ascii_whitespace()
        .enumerate()
        .filter(|(i, _)| *i >= 1)
        .map(|(_, a)| a.parse::<f64>().unwrap())
        .collect::<Vec<f64>>();

    println!("VALOR A PAGAR: R$ {:.2}", peca1[0] * peca1[1] + peca2[0] * peca2[1]);
}

use std::io;

fn main() {
    let mut numeros = String::new();
    io::stdin().read_line(&mut numeros).unwrap();
    let numeros = numeros
        .split_ascii_whitespace()
        .map(|a| a.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    
    let a = numeros.get(0).unwrap();
    let b = numeros.get(1).unwrap();
    let c = numeros.get(2).unwrap();

    let t = (a + b + (a - b).abs()) / 2;
    println!("{} eh o maior", (t + c + (t - c).abs()) / 2);
}

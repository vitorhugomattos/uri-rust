use std::io;

fn main() {
    let mut number = String::new();
    let mut hours = String::new();
    let mut salary = String::new();

    io::stdin().read_line(&mut number).unwrap();
    io::stdin().read_line(&mut hours).unwrap();
    io::stdin().read_line(&mut salary).unwrap();

    let number = number.trim().parse::<u32>().unwrap();
    let hours = hours.trim().parse::<f64>().unwrap();
    let salary = salary.trim().parse::<f64>().unwrap();

    println!("NUMBER = {}", number);
    println!("SALARY = U$ {:.2}", hours * salary);
}

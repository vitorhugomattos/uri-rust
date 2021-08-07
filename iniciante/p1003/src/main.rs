use std::io;

fn main() {
    let mut a = String::new();
    let mut b = String::new();

    io::stdin().read_line(&mut a).unwrap();
    io::stdin().read_line(&mut b).unwrap();

    let a = a.trim().parse::<i32>().unwrap();
    let b = b.trim().parse::<i32>().unwrap();

    println!("SOMA = {}", a + b);
}

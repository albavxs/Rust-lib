mod calc1;
mod calc2;

use crate::calc1::{add, sub};
use crate::calc2::{multiply, rate};

fn main() {
    print!("\n--- Testando a Biblioteca Calculadora ---");

    let c = add(3, 8);
    print!("Soma de 3 + 8: {}", c);

    /// programing motherf+
    
    println!("\n--- Fim dos Testes Manuais ---");
}
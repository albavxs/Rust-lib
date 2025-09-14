mod calc1;
mod calc2;
mod calc3;

use crate::calc1::{add, sub};
use crate::calc2::{multiply, rate};
use crate::calc3::{potencia, log};

fn main() {
    print!("\n--- Testando a Biblioteca Calculadora ---");

    let c = add(3, 8);
    print!("Soma de 3 + 8: {}", c);

    let d = sub(10, 4);
    println!("Subtração de 10 - 4: {}", d);

    let e = multiply(6, 7);
    println!("Multiplicação de 6 * 7: {}", e);

    let f = rate(20, 4);
    println!("Divisão de 20 / 4: {}", f);

    let g = potencia(2.0, 3.0);
    println!("Potência 2^3 = {}", g);

    let h = log(25.0, 5.0);
    println!("Logaritmo log_5(25) = {}", h);
    /// programing motherf+
    
    println!("\n--- Fim dos Testes Manuais ---");
}
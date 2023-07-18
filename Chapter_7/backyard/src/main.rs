use crate::garden::vegetables::Asparagus;

pub mod garden; // Строка pub mod garden; говорит компилятору о подключении кода, найденном в src/garden.rs:

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}
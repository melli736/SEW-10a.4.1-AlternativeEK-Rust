use std::io;

/// Führt einfache Operationen mit einem Vektor aus: Zahlen eingeben, anzeigen, summieren.
pub fn run() {
    let mut numbers: Vec<i32> = Vec::new();

    numbers.push(2);
    numbers.push(5);
    numbers.push(8);
    numbers.push(1);


    println!("Eingegebene Zahlen:");
    for num in &numbers {
        println!("{}", num);
    }

    let sum: i32 = numbers.iter().sum();
    println!("Summe: {}", sum);
}

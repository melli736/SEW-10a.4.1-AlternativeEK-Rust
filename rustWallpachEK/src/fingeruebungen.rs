use std::io;

/// Führt einfache Operationen mit einem Vektor aus: Zahlen eingeben, anzeigen, summieren.
pub fn run() {
    let mut numbers: Vec<i32> = Vec::new();

    println!("Gib ein paar Zahlen ein (leere Zeile zum Beenden):");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input.is_empty() {
            break;
        }
        if let Ok(n) = input.parse::<i32>() {
            numbers.push(n);
        } else {
            println!("Ungültige Zahl");
        }
    }

    println!("Eingegebene Zahlen:");
    for num in &numbers {
        println!("{}", num);
    }

    let sum: i32 = numbers.iter().sum();
    println!("Summe: {}", sum);
}

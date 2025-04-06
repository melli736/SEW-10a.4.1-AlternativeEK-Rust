# SEW-10a.4.1-AlternativeEK-Rust
**Author:** Melissa Wallpach 5BHIT

Dieses Projekt besteht aus vier voneinander unabhängigen Rust-Modulen, die typische Einstiegsthemen abdecken: Arbeiten mit Vektoren, Structs & Methoden und Enums.

Es eignet sich ideal zum Lernen und Üben mit der IDE **Visual Studio Code** und der Erweiterung **rust-analyzer**.



## Aufbau

```
rust_proj/
├── Cargo.toml
└── src/
    ├── main.rs             // Einstiegspunkt
    ├── fingeruebungen.rs        // Fingerübungen mit Vektoren
    ├── structs.rs    // Structs & Methoden
    └── enums.rs      // Enums: Schere-Stein-Papier
```

## Ausführen

### Voraussetzungen

- [Rust installieren](https://www.rust-lang.org/tools/install)  
- Optional: VS Code + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

### Projekt kompilieren und ausführen

```
cargo run
```



## Code

### Teil 1: Fingerübungen mit Vektor

- Einlesen von Zahlen über die Konsole
- Speicherung in einem `Vec`
- Ausgabe aller Zahlen
- Berechnung der Summe

Siehe fingeruebungen.rs

### Teil 2: Structs & Methoden

- Struct `Point` mit privaten Feldern `x` und `y`
- Methoden: `new`, `get`, `set`, `translate`, `distance`, `abs`
- Beispiel: Punkte verschieben und Abstände berechnen

Siehe `structs.rs`

### Teil 3: Enums – Schere, Stein, Papier

- Enum `Move` (Schere, Stein, Papier)
- Enum `Outcome` (Gewonnen, Verloren, Unentschieden)
- Methoden zur Ergebnisberechnung und Perspektivwechsel

Siehe `enums.rs`

## Tests

Tests können bei Bedarf in den einzelnen Modulen ergänzt werden, z. B. mit `#[cfg(test)]`.



## Referenzen

- [The Rust Programming Language Buch (offiziell)](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Standardbibliothek – Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html)
- [Standardbibliothek – Option](https://doc.rust-lang.org/std/option/)
/// Mögliche Spielzüge bei Schere-Stein-Papier.
#[derive(Debug, Copy, Clone, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

/// Ergebnis einer Runde Schere-Stein-Papier.
#[derive(Debug, Copy, Clone, PartialEq)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Move {
    /// Spielt gegen einen anderen Zug und liefert das Ergebnis aus Sicht von `self`.
    fn play_against(&self, other: Move) -> Outcome {
        use Move::*;
        use Outcome::*;
        match (*self, other) {
            (a, b) if a == b => Draw,
            (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => Win,
            _ => Lose,
        }
    }
}

impl Outcome {
    /// Kehrt die Perspektive des Ergebnisses um.
    fn reverse(&self) -> Outcome {
        use Outcome::*;
        match self {
            Win => Lose,
            Lose => Win,
            Draw => Draw,
        }
    }
}

pub fn run() {
    let mine = Move::Rock;
    let theirs = Move::Scissors;

    let result = mine.play_against(theirs);
    println!("Ich: {:?}, Gegner: {:?} → Ergebnis: {:?}", mine, theirs, result);
    println!("Aus Gegner-Sicht: {:?}", result.reverse());
}

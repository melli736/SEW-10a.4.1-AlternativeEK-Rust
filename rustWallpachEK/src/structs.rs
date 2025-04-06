/// Ein Punkt im 2D-Raum
#[derive(Debug, Copy, Clone)]
pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    /// Erzeugt einen neuen Punkt mit den gegebenen Koordinaten.
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Gibt die Koordinaten des Punkts zurück.
    pub fn get(&self) -> (f64, f64) {
        (self.x, self.y)
    }

    /// Setzt den Punkt auf neue Koordinaten.
    pub fn set(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }

    /// Verschiebt den Punkt um (dx, dy).
    pub fn translate(&mut self, dx: f64, dy: f64) {
        self.x += dx;
        self.y += dy;
    }

    /// Berechnet die Entfernung zu einem anderen Punkt.
    pub fn distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    /// Berechnet die Entfernung zum Ursprung (0, 0).
    pub fn abs(&self) -> f64 {
        self.distance(&Point::new(0.0, 0.0))
    }
}

pub fn run() {
    let mut p = Point::new(3.0, 4.0);
    println!("{:?}", p);

    p.translate(1.0, 2.0);
    println!("Nach Verschiebung: {:?}", p);
    println!("Abstand zum Ursprung: {}", p.abs());

    let q = Point::new(0.0, 0.0);
    println!("Abstand zu Punkt {:?}: {}", q, p.distance(&q));
}

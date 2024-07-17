


struct Bezier {
    pub p0: Point,
    pub p1: Point,
    pub p2: Point,
    pub p3: Point,
}

struct Point {
    pub x: f64,
    pub y: f64,
}

enum Beziers {
    Cubic(Bezier),
    Quadratic(Bezier),
    Line(Bezier),
}
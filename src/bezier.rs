#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub(crate) enum Bezier {
    Cubic([Point; 4]),
    Quadratic([Point; 3]),
    Line([Point; 2]),
}

impl Bezier {
    pub fn new_c(origin: Point, p0: Point, p1: Point, p2: Point) -> Self {
        Bezier::Cubic([origin, p0, p1, p2])
    }

    pub fn new_q(origin: Point, p0: Point, p1: Point) -> Self {
        Bezier::Quadratic([origin, p0, p1])
    }

    pub fn new_l(p0: Point, p1: Point) -> Self {
        Bezier::Line([p0, p1])
    }

    pub fn point_at(&self, t: f64) -> Result<Point, &'static str> {
        if t > 1.0 || t < 0.0 {
            return Err("t must be between 0 and 1");
        }
        match self {
            Bezier::Cubic(points) => {
                let x = (1.0 - t).powi(3) * points[0].x + 3.0 * (1.0 - t).powi(2) * t * points[1].x + 3.0 * (1.0 - t) * t.powi(2) * points[2].x + t.powi(3) * points[3].x;
                let y = (1.0 - t).powi(3) * points[0].y + 3.0 * (1.0 - t).powi(2) * t * points[1].y + 3.0 * (1.0 - t) * t.powi(2) * points[2].y + t.powi(3) * points[3].y;
                Ok(Point { x, y })
            },
            Bezier::Quadratic(points) => {
                let x = (1.0 - t).powi(2) * points[0].x + 2.0 * (1.0 - t) * t * points[1].x + t.powi(2) * points[2].x;
                let y = (1.0 - t).powi(2) * points[0].y + 2.0 * (1.0 - t) * t * points[1].y + t.powi(2) * points[2].y;
                Ok(Point { x, y })
            },
            Bezier::Line(points) => {
                let x = (1.0 - t) * points[0].x + t * points[1].x;
                let y = (1.0 - t) * points[0].y + t * points[1].y;
                Ok(Point { x, y })
            },
        }
    }

    pub fn get_origin(&self) -> Point {
        match self {
            Bezier::Cubic(points) => points[0],
            Bezier::Quadratic(points) => points[0],
            Bezier::Line(points) => points[0],
        }
    }

    pub fn get_points(&self) -> Vec<Point> {
        match self {
            Bezier::Cubic(points) => points.to_vec(),
            Bezier::Quadratic(points) => points.to_vec(),
            Bezier::Line(points) => points.to_vec(),
        }
    }

    pub fn curvature_at_point(&self, t: f64) -> Result<Point, &'static str> {
        if t > 1.0 || t < 0.0 {
            return Err("t must be between 0 and 1");
        }
        match self {
            Bezier::Cubic(points) => {
                let x = 3.0 * (1.0 - t).powi(2) * (points[1].x - points[0].x) + 6.0 * (1.0 - t) * t * (points[2].x - points[1].x) + 3.0 * t.powi(2) * (points[3].x - points[2].x);
                let y = 3.0 * (1.0 - t).powi(2) * (points[1].y - points[0].y) + 6.0 * (1.0 - t) * t * (points[2].y - points[1].y) + 3.0 * t.powi(2) * (points[3].y - points[2].y);
                Ok(Point { x, y })
            },
            Bezier::Quadratic(points) => {
                let x = 2.0 * (1.0 - t) * (points[1].x - points[0].x) + 2.0 * t * (points[2].x - points[1].x);
                let y = 2.0 * (1.0 - t) * (points[1].y - points[0].y) + 2.0 * t * (points[2].y - points[1].y);
                Ok(Point { x, y })
            },
            Bezier::Line(points) => {
                let x = points[1].x - points[0].x;
                let y = points[1].y - points[0].y;
                Ok(Point { x, y })
            },
        }
    }
}
mod bezier;
mod interpreter;

use bezier::Point;
use bezier::Bezier;

fn main() {
    // test beziers
    let cubic = Bezier::new_c(Point { x: 0.0, y: 0.0 }, Point { x: 1.0, y: 1.0 }, Point { x: 2.0, y: 1.0 }, Point { x: 3.0, y: 0.0 });
    let quadratic = Bezier::new_q(Point { x: 0.0, y: 0.0 }, Point { x: 1.0, y: 1.0 }, Point { x: 2.0, y: 0.0 });
    let line = Bezier::new_l(Point { x: 0.0, y: 0.0 }, Point { x: 1.0, y: 1.0 });
    
    let cubic_points = (0..=10).map(|i| cubic.point_at(i as f64 / 10.0).unwrap()).collect::<Vec<Point>>();
    let quadratic_points = (0..=10).map(|i| quadratic.point_at(i as f64 / 10.0).unwrap()).collect::<Vec<Point>>();
    let line_points = (0..=10).map(|i| line.point_at(i as f64 / 10.0).unwrap()).collect::<Vec::<Point>>();
    
    println!("{:?}", cubic_points);
    println!("{:?}", quadratic_points);
    println!("{:?}", line_points);

    interpreter::parse_svg(interpreter::read_in("./res/output.svg"));
}

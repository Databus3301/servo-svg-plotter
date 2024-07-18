mod bezier;
mod interpreter;

use bezier::Point;
use bezier::Bezier;

fn main() {
    interpreter::parse_svg(interpreter::read_in("./res/output.svg"));
}

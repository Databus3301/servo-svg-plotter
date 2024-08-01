mod bezier;
mod interpreter;

use bezier::Point;
use bezier::Bezier;

fn main() {

    //interpreter::parse_svg(interpreter::read_in("./res/tests/relative_absolute.svg"));
    //interpreter::parse_svg(interpreter::read_in("./res/tests/relative_decimals.svg"));
    interpreter::parse_svg(interpreter::read_in("./res/tests/decimal_curves.svg"));

}

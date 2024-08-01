mod bezier;
mod interpreter;


fn main() {

    //interpreter::parse_svg(interpreter::read_in("./res/tests/relative_absolute.svg"));
    //interpreter::parse_svg(interpreter::read_in("./res/tests/relative_decimals.svg"));
    interpreter::parse_svg(interpreter::read_in("./res/tests/implied_lines.svg"));

}

mod bezier;
mod interpreter;


fn main() {

    interpreter::parse_svg(interpreter::read_in("./res/tests/smooth_curves.svg"));

}

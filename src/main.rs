mod bezier;
mod interpreter;


fn main() {

    let bs = interpreter::parse_paths(interpreter::read_paths("./res/tests/multi_move.svg"));

}

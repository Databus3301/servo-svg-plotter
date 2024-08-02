mod bezier;
mod interpreter;


fn main() {

    let bs = interpreter::parse_svg(interpreter::read_in("./res/out.svg"));
    println!("bezier count: {}", bs.len());


}

use std::io::{BufRead, BufReader};
use std::str::FromStr;
use crate::bezier;
use crate::bezier::Point;
use crate::bezier::Bezier;

pub fn read_in(path: &str) -> Vec<String> {
    let br = BufReader::new(std::fs::File::open(path).unwrap());
    br.lines()
        .filter_map(|line_result| {
            line_result.ok().and_then(|line| {
                if line.contains(" d=\"") { Some(line) } else { None }
            })
        })
        .collect()
}

pub fn parse_svg(svg: Vec<String>) -> Vec<Bezier> {
    let beziers: Vec<Bezier> = vec!();

    let mut state: ParseState = ParseState::Start;
    let mut cur_content = String::new();
    let start;
    let last_move = Point { x: 0.0, y: 0.0 };
    let last_bezier: Bezier;

    for l in svg {
        for c in l.chars() {
            match c.to_ascii_lowercase() {
                'm' => {

                    state = ParseState::Move
                },
                'c' => state = ParseState::Cubic,
                'q' => state = ParseState::Quadratic,
                'l' => state = ParseState::Line,
                _ => cur_content.push(c)
            }
        }
    }
    beziers
}

fn resolve_content(state: ParseState, content: String, last_move: Point) {

}


#[derive(Debug, PartialEq)]
enum ParseState {
    Start,
    Move,
    Cubic,
    Quadratic,
    Line,
}
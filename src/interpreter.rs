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
    let mut beziers: Vec<Bezier> = vec!();

    let mut state: ParseState = ParseState::Start;
    let mut cur_content = String::new();
    let mut start: Option<Point> = None;
    let mut last_pos = Point { x: 0.0, y: 0.0 };
    let last_bezier: Bezier;

    let resolve_move = |content: &str| {

        let mut split = content.split_whitespace().peekable();
        let x = f64::from_str(split.next().unwrap()).unwrap();
        let y = f64::from_str(split.next().unwrap()).unwrap();
        last_pos = Point { x, y };

        if start.is_none() {
            start = Some(last_pos);
        }

        // handle implicit line syntax i.e -> m 1 1 2 2 3 3 z
        if split.peek().is_some() {
            let x = f64::from_str(split.next().unwrap()).unwrap();
            let y = f64::from_str(split.next().unwrap()).unwrap();
            last_pos = Point { x, y };
            last_bezier = Bezier::new_l(start.unwrap(), last_pos, last_pos);
            beziers.push(last_bezier);
        }
    };

    let resolve_cubic = |content: &str| {
        let mut split = content.split_whitespace();
        let x0 = f64::from_str(split.next().unwrap()).unwrap();
        let y0 = f64::from_str(split.next().unwrap()).unwrap();
        let x1 = f64::from_str(split.next().unwrap()).unwrap();
        let y1 = f64::from_str(split.next().unwrap()).unwrap();
        let x2 = f64::from_str(split.next().unwrap()).unwrap();
        let y2 = f64::from_str(split.next().unwrap()).unwrap();
        let x3 = f64::from_str(split.next().unwrap()).unwrap();
        let y3 = f64::from_str(split.next().unwrap()).unwrap();
        let origin = last_pos;
        last_bezier = Bezier::new_c(origin, Point { x: x0, y: y0 }, Point { x: x1, y: y1 }, Point { x: x2, y: y2 }, Point { x: x3, y: y3 });
        beziers.push(last_bezier);
    };

    let resolve_quadratic = |content: &str| {
        let mut split = content.split_whitespace();
        let x0 = f64::from_str(split.next().unwrap()).unwrap();
        let y0 = f64::from_str(split.next().unwrap()).unwrap();
        let x1 = f64::from_str(split.next().unwrap()).unwrap();
        let y1 = f64::from_str(split.next().unwrap()).unwrap();
        let x2 = f64::from_str(split.next().unwrap()).unwrap();
        let y2 = f64::from_str(split.next().unwrap()).unwrap();
        let origin = last_pos;
        last_bezier = Bezier::new_q(origin, Point { x: x0, y: y0 }, Point { x: x1, y: y1 }, Point { x: x2, y: y2 });
        beziers.push(last_bezier);
    };

    let resolve_line = |content: &str| {
        let mut split = content.split_whitespace();
        let x0 = f64::from_str(split.next().unwrap()).unwrap();
        let y0 = f64::from_str(split.next().unwrap()).unwrap();
        let x1 = f64::from_str(split.next().unwrap()).unwrap();
        let y1 = f64::from_str(split.next().unwrap()).unwrap();
        let origin = last_pos;
        last_bezier = Bezier::new_l(origin, Point { x: x0, y: y0 }, Point { x: x1, y: y1 });
        beziers.push(last_bezier);
    };



    for l in svg {
        for c in l.chars() {
            match c.to_ascii_lowercase() {
                'm' => {
                    resolve_move(&cur_content);
                    state = ParseState::Move
                },
                'c' => {
                    resolve_cubic(&cur_content);
                    state = ParseState::Cubic
                },
                'q' => {
                    resolve_quadratic(&cur_content);
                    state = ParseState::Quadratic
                },
                'l' => {
                    resolve_line(&cur_content);
                    state = ParseState::Line
                },
                _ => cur_content.push(c)
            }
        }
    }



    beziers
}




#[derive(Debug, PartialEq)]
enum ParseState {
    Start,
    Move,
    Cubic,
    Quadratic,
    Line,
}
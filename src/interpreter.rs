use std::io::{BufRead, BufReader};
use std::iter::Peekable;
use std::str::{FromStr, SplitWhitespace};
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

    let mut state: ParseState = ParseState::Read;
    let mut cur_content = String::new();
    let mut start: Option<Point> = None;
    let mut last_pos = Point { x: 0.0, y: 0.0 };
    let mut last_bezier: Bezier = Bezier::new_l(Point { x: 0.0, y: 0.0 }, Point { x: 0.0, y: 0.0 });

    let mut resolve_path = |state: &ParseState, c: &char| {
        let mut split = cur_content.split_whitespace().peekable();
        match state {
            ParseState::Move => {
                let x = parse_f64(&mut split);
                let y = parse_f64(&mut split);
                last_pos = Point { x, y };

                if start.is_none() {
                    start = Some(last_pos);
                }

                while split.peek().is_some() {
                    let x = parse_f64(&mut split);
                    let y = parse_f64(&mut split);
                    last_bezier = Bezier::new_l(last_pos, Point { x, y });
                    last_pos = Point { x, y };
                    beziers.push(last_bezier.clone());
                }
            },
            ParseState::Cubic => {
                let x0 = parse_f64(&mut split);
                let y0 = parse_f64(&mut split);
                let x1 = parse_f64(&mut split);
                let y1 = parse_f64(&mut split);
                let x2 = parse_f64(&mut split);
                let y2 = parse_f64(&mut split);
                let x3 = parse_f64(&mut split);
                let y3 = parse_f64(&mut split);
                let origin = last_pos;
                last_bezier = Bezier::new_c(origin, Point { x: x0, y: y0 }, Point { x: x1, y: y1 }, Point { x: x2, y: y2 }, Point { x: x3, y: y3 });
                last_pos = last_bezier.point_at(1f64).unwrap();
                beziers.push(last_bezier);
            },
            ParseState::Quadratic => {
                let x0 = parse_f64(&mut split);
                let y0 = parse_f64(&mut split);
                let x1 = parse_f64(&mut split);
                let y1 = parse_f64(&mut split);
                let x2 = parse_f64(&mut split);
                let y2 = parse_f64(&mut split);
                let origin = last_pos;
                last_bezier = Bezier::new_q(origin, Point { x: x0, y: y0 }, Point { x: x1, y: y1 }, Point { x: x2, y: y2 });
                last_pos = last_bezier.point_at(1f64).unwrap();
                beziers.push(last_bezier);
            },
            ParseState::Line => {
                let x0 = parse_f64(&mut split);
                let y0 = parse_f64(&mut split);
                let x1 = parse_f64(&mut split);
                let y1 = parse_f64(&mut split);
                last_bezier = Bezier::new_l(Point { x: x0, y: y0 }, Point { x: x1, y: y1 });
                last_pos = Point { x: x1, y: y1 };
                beziers.push(last_bezier);
            },
            ParseState::Read => {
                if c.is_whitespace() {
                    return;
                }
                cur_content.push(*c);
            }
        }
    };

    fn parse_f64(mut split: &mut Peekable<SplitWhitespace>) -> f64 {
        f64::from_str(split.next().unwrap().trim()).unwrap()
    }



    for l in svg {
        for c in l.chars() {
            match c.to_ascii_lowercase() {
                'm' => {
                    resolve_path(&state, &c);
                    state = ParseState::Move;
                },
                'c' => {
                    resolve_path(&state, &c);
                    state = ParseState::Cubic;
                },
                'q' => {
                    resolve_path(&state, &c);
                    state = ParseState::Quadratic;
                },
                'l' => {
                    resolve_path(&state, &c);
                    state = ParseState::Line;
                },
                _ => resolve_path(&ParseState::Read, &c),
            }
        }
    }



    beziers
}




#[derive(Debug, PartialEq)]
enum ParseState {
    Read,
    Move,
    Cubic,
    Quadratic,
    Line,
}
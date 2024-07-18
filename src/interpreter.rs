use std::io::{BufRead, BufReader};
use std::iter::Peekable;
use std::str::{FromStr, SplitWhitespace};
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
        match state {
            ParseState::Move => {
                let nums = tokenize(&cur_content);
                println!("{:?}", nums);

                for i in (0..nums.len()).step_by(2) {
                    let x = nums[i];
                    let y = nums[i+1];
                    last_pos = Point { x, y };

                    if start.is_none() {
                        start = Some(last_pos);
                    }

                    if i != 0 {
                        last_bezier = Bezier::new_l(last_pos, Point { x, y });
                        beziers.push(last_bezier.clone());
                    }
                }
            },
            ParseState::Cubic => {
                let nums = tokenize(&cur_content);
                let origin = last_pos;
                last_bezier = Bezier::new_c(origin, Point { x: nums[0], y: nums[1] }, Point { x: nums[2], y: nums[3] }, Point { x: nums[4], y: nums[5] }, Point { x: nums[6], y: nums[7] });
                last_pos = last_bezier.point_at(1f64).unwrap();
                beziers.push(last_bezier);
                cur_content.clear();
            },
            ParseState::Quadratic => {
                let nums = tokenize(&cur_content);
                let origin = last_pos;
                last_bezier = Bezier::new_q(origin, Point { x: nums[0], y: nums[1] }, Point { x: nums[2], y: nums[3] }, Point { x: nums[4], y: nums[5] });
                last_pos = last_bezier.point_at(1f64).unwrap();
                beziers.push(last_bezier);
                cur_content.clear();
            },
            ParseState::Line => {
                let nums = tokenize(&cur_content);
                last_bezier = Bezier::new_l(Point { x: nums[0], y: nums[1] }, Point { x: nums[2], y: nums[3] });
                last_pos = Point { x: nums[2], y: nums[3] };
                beziers.push(last_bezier);
                cur_content.clear();
            },
            ParseState::Horizontal => {
                let nums = tokenize(&cur_content);
                last_bezier = Bezier::new_l(last_pos, Point { x: nums[0], y: last_pos.y });
                last_pos = Point { x: nums[0], y: last_pos.y };
                beziers.push(last_bezier);
                cur_content.clear();
            },
            ParseState::Vertical => {
                let nums = tokenize(&cur_content);
                last_bezier = Bezier::new_l(last_pos, Point { x: last_pos.x, y: nums[0] });
                last_pos = Point { x: last_pos.x, y: nums[0] };
                beziers.push(last_bezier);
                cur_content.clear();
            },
            ParseState::Read => {
                cur_content.push(*c);
            }
        }
    };

    fn tokenize(content: &str) -> Vec<f64> {
        content.split(|c: char| !c.is_numeric() || c == '.')
            .filter(|s| !s.is_empty())
            .map(|s| f64::from_str(s.trim()).unwrap_or(0.0))
            .collect::<Vec<f64>>()
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
                'h' => {
                    resolve_path(&state, &c);
                    state = ParseState::Horizontal;
                },
                'v' => {
                    resolve_path(&state, &c);
                    state = ParseState::Vertical;
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
    Horizontal,
    Vertical,
}
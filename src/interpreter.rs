use std::f64::consts::PI;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
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

pub fn parse_svg(mut svg: Vec<String>) -> Vec<Bezier> {
    let mut beziers: Vec<Bezier> = vec!();

    let mut state: ParseState = ParseState::Read;
    let mut cur_content = String::new();
    let mut start: Option<Point> = None;
    let mut last_pos = Point { x: 0.0, y: 0.0 };
    let mut last_bezier: Bezier = Bezier::new_l(Point { x: 0.0, y: 0.0 }, Point { x: 0.0, y: 0.0 });

    let mut resolve_path = |state: &ParseState, c: &char| {
        match state {
            ParseState::MOVE => {
                let nums = tokenize(&cur_content);

                for i in (0..nums.len()).step_by(2) {
                    let x = nums[i];
                    let y = nums[i+1];

                    if i != 0 {
                        last_bezier = Bezier::new_l(last_pos, Point { x, y });
                        beziers.push(last_bezier.clone());
                        last_pos = Point { x, y };

                        log("Line", last_bezier, start);
                    } else {
                        last_pos = Point { x, y };
                        start = Some(last_pos);
                        println!("Start: {:?}", start);
                    }
                }
                cur_content.clear();
            },
            ParseState::Move => {
                let nums = tokenize(&cur_content);

                for i in (0..nums.len()).step_by(2) {
                    let x = last_pos.x + nums[i];
                    let y = last_pos.y + nums[i+1];

                    if i != 0 {
                        last_bezier = Bezier::new_l(last_pos, Point {  x, y });
                        last_pos = Point { x, y };
                        beziers.push(last_bezier.clone());
                        log("Line", last_bezier, start);
                    } else {
                        last_pos = Point { x, y };
                        start = Some(last_pos);
                        println!("Start: {:?}", start);
                    }
                }
                cur_content.clear();
            }
            ParseState::CUBIC => {
                let nums = tokenize(&cur_content);
                let origin = last_pos;
                last_bezier = Bezier::new_c(origin, Point { x: nums[0], y: nums[1] }, Point { x: nums[2], y: nums[3] }, Point { x: nums[4], y: nums[5] });
                last_pos = last_bezier.point_at(1f64).unwrap();
                beziers.push(last_bezier);
                cur_content.clear();

                log("Cubic", last_bezier, start);
            },
            ParseState::QUADRATIC => {
                let nums = tokenize(&cur_content);
                let origin = last_pos;
                last_bezier = Bezier::new_q(origin, Point { x: nums[0], y: nums[1] }, Point { x: nums[2], y: nums[3] });
                last_pos = last_bezier.point_at(1f64).unwrap();
                beziers.push(last_bezier);
                cur_content.clear();

                log("Quadratic", last_bezier, start);
            },
            ParseState::LINE => {
                let nums = tokenize(&cur_content);
                last_bezier = Bezier::new_l(last_pos, Point { x: nums[0], y: nums[1] });
                last_pos = Point { x: nums[0], y: nums[1] };
                beziers.push(last_bezier);
                cur_content.clear();

                log("Line", last_bezier, start);
            },
            ParseState::HORIZONTAL => {
                let nums = tokenize(&cur_content);
                last_bezier = Bezier::new_l(last_pos, Point { x: nums[0], y: last_pos.y });
                last_pos = Point { x: nums[0], y: last_pos.y };
                beziers.push(last_bezier);
                cur_content.clear();

                log("Horizontal", last_bezier, start);
            },
            ParseState::VERTICAL => {
                let nums = tokenize(&cur_content);
                last_bezier = Bezier::new_l(last_pos, Point { x: last_pos.x, y: nums[0] });
                last_pos = Point { x: last_pos.x, y: nums[0] };
                beziers.push(last_bezier);
                cur_content.clear();

                log("Vertical", last_bezier, start);
            },
            ParseState::ARC => {
                let nums = tokenize(&cur_content);
                let origin = last_pos;
                let rx = nums[0];
                let ry = nums[1];
                let x_axis_rotation = nums[2];
                let large_arc_flag = nums[3] == 1.0;
                let sweep_flag = nums[4] == 1.0;
                let end = Point { x: nums[5], y: nums[6] };
                let arcs = arc_to_beziers(origin, rx, ry, x_axis_rotation, large_arc_flag, sweep_flag, end);
                for a in arcs {
                    beziers.push(a);
                    log("Arc", a, start);
                }
                last_pos = end;
                cur_content.clear();

            },
            ParseState::CLOSE => {
                if let Some(s) = start {
                    last_bezier = Bezier::new_l(last_pos, s);
                    last_pos = s;
                    beziers.push(last_bezier);
                    start = None;
                }
                cur_content.clear();
                log("Close", last_bezier, start);
                println!();
            },

            _ => {
                cur_content.push(*c);
                if *c == '$' {
                    cur_content.clear();
                }
            }
        }
    };

    fn tokenize(content: &str) -> Vec<f64> {
       let nums = content.split(|c: char| !c.is_ascii_digit() && c != '.' && c != '-')
            .filter(|s| !s.is_empty())
            .map(|s| f64::from_str(s.trim()).unwrap_or(-42.4242))
            .filter(|n| *n != -42.4242)
            .collect::<Vec<f64>>();

        nums
    }


    // replace all '-' with " -"
    for l in svg.iter_mut() {
        *l = l.replace("-", " -");
        *l = l.replace("h", " h");
    }

    // for each d= attribute in the svg collect all the data for each path command then parse it
    for l in svg {
        for c in l.chars() {
            match c {
                'M' => {
                    resolve_path(&state, &c);
                    state = ParseState::MOVE;
                },
                'C' => {
                    resolve_path(&state, &c);
                    state = ParseState::CUBIC;
                },
                'Q' => {
                    resolve_path(&state, &c);
                    state = ParseState::QUADRATIC;
                },
                'L' => {
                    resolve_path(&state, &c);
                    state = ParseState::LINE;
                },
                'H' => {
                    resolve_path(&state, &c);
                    state = ParseState::HORIZONTAL;
                },
                'V' => {
                    resolve_path(&state, &c);
                    state = ParseState::VERTICAL;
                },
                'A' => {
                    resolve_path(&state, &c);
                    state = ParseState::ARC;
                },
                'Z' => {
                    resolve_path(&state, &c);
                    state = ParseState::CLOSE;
                },


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
                'a' => {
                    resolve_path(&state, &c);
                    state = ParseState::Arc;
                },
                'z' => {
                    resolve_path(&state, &c);
                    state = ParseState::Close;
                },
                _ => resolve_path(&ParseState::Read, &c),
            }
        }
        resolve_path(&state, &'$');
    }
    beziers
}




#[derive(Debug, PartialEq)]
enum ParseState {
    Read,
    MOVE,
    Move,
    Cubic,
    CUBIC,
    Quadratic,
    QUADRATIC,
    Line,
    LINE,
    Horizontal,
    HORIZONTAL,
    Vertical,
    VERTICAL,
    Arc,
    ARC,
    Close,
    CLOSE
}

fn log(titel: &str, content: Bezier, start: Option<Point>) {
    if start.is_some() {
        print!("    ");
    }
    println!("{}: {:?}", titel, content);
}

// THE FOLLOWING IS GENERATE BY AI (GPT4o)

fn arc_to_beziers(start: Point, rx: f64, ry: f64, x_axis_rotation: f64, large_arc_flag: bool, sweep_flag: bool, end: Point) -> Vec<Bezier> {
    let mut beziers = Vec::new();

    // Convert angles from degrees to radians
    let x_axis_rotation = x_axis_rotation * PI / 180.0;

    // Calculate the distances between the start and end points
    let dx2 = (start.x - end.x) / 2.0;
    let dy2 = (start.y - end.y) / 2.0;

    // Calculate the transformed start point
    let x1p = dx2 * x_axis_rotation.cos() + dy2 * x_axis_rotation.sin();
    let y1p = -dx2 * x_axis_rotation.sin() + dy2 * x_axis_rotation.cos();

    // Ensure radii are large enough
    let rx = rx.abs();
    let ry = ry.abs();
    let lambda = (x1p * x1p) / (rx * rx) + (y1p * y1p) / (ry * ry);

    let (rx, ry) = if lambda > 1.0 {
        let lambda_sqrt = lambda.sqrt();
        (rx * lambda_sqrt, ry * lambda_sqrt)
    } else {
        (rx, ry)
    };

    // Calculate the center
    let sign = if large_arc_flag == sweep_flag { -1.0 } else { 1.0 };
    let sq = ((rx * rx * ry * ry) - (rx * rx * y1p * y1p) - (ry * ry * x1p * x1p)) / ((rx * rx * y1p * y1p) + (ry * ry * x1p * x1p));
    let sq = if sq < 0.0 { 0.0 } else { sq };
    let coef = sign * sq.sqrt();
    let cxp = coef * ((rx * y1p) / ry);
    let cyp = coef * -((ry * x1p) / rx);

    // Calculate the actual center point
    let cx = (start.x + end.x) / 2.0 + x_axis_rotation.cos() * cxp - x_axis_rotation.sin() * cyp;
    let cy = (start.y + end.y) / 2.0 + x_axis_rotation.sin() * cxp + x_axis_rotation.cos() * cyp;

    // Calculate the start angle and the extent angle
    let theta1 = angle(1.0, 0.0, (x1p - cxp) / rx, (y1p - cyp) / ry);
    let dtheta = angle((x1p - cxp) / rx, (y1p - cyp) / ry, (-x1p - cxp) / rx, (-y1p - cyp) / ry) % (2.0 * PI);

    let segments = if large_arc_flag {
        if dtheta < 0.0 {
            2.0 * PI + dtheta
        } else {
            dtheta
        }
    } else {
        if dtheta > 0.0 {
            dtheta - 2.0 * PI
        } else {
            dtheta
        }
    }.abs() / (PI / 2.0).ceil();

    // Split the arc into segments and convert each to a Bezier curve
    for i in 0..segments as usize {
        let theta2 = theta1 + dtheta * ((i + 1) as f64) / segments;
        beziers.push(convert_arc_segment_to_bezier(cx, cy, rx, ry, x_axis_rotation, theta1 + dtheta * i as f64 / segments, theta2));
    }

    beziers
}

fn angle(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    let dot = x1 * x2 + y1 * y2;
    let det = x1 * y2 - y1 * x2; // determinant
    det.atan2(dot)
}

fn convert_arc_segment_to_bezier(cx: f64, cy: f64, rx: f64, ry: f64, x_axis_rotation: f64, start_angle: f64, end_angle: f64) -> Bezier {
    let delta_angle = end_angle - start_angle;
    let t = (4.0 / 3.0) * (delta_angle / 2.0).tan();

    let sin_start = start_angle.sin();
    let cos_start = start_angle.cos();
    let sin_end = end_angle.sin();
    let cos_end = end_angle.cos();

    let e1x = -rx * cos_start - ry * sin_start * t;
    let e1y = -ry * sin_start + rx * cos_start * t;
    let e2x = rx * cos_end + ry * sin_end * t;
    let e2y = ry * sin_end - rx * cos_end * t;

    let start_x = cx + rx * cos_start * x_axis_rotation.cos() - ry * sin_start * x_axis_rotation.sin();
    let start_y = cy + rx * cos_start * x_axis_rotation.sin() + ry * sin_start * x_axis_rotation.cos();
    let end_x = cx + rx * cos_end * x_axis_rotation.cos() - ry * sin_end * x_axis_rotation.sin();
    let end_y = cy + rx * cos_end * x_axis_rotation.sin() + ry * sin_end * x_axis_rotation.cos();

    Bezier::Cubic([
        Point { x: start_x, y: start_y },
        Point { x: start_x + e1x, y: start_y + e1y },
        Point { x: end_x + e2x, y: end_y + e2y },
        Point { x: end_x, y: end_y },
    ])
}
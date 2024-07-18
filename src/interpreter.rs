use std::io::{BufRead, BufReader};
use std::str::FromStr;
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
    for l in svg {
        println!("{}", l.trim());
    }
    println!("{:?}", f64::from_str("0.0 ".trim()));
    vec!()
}
use std::io::{BufRead, BufReader};
use crate::bezier::Bezier;

pub fn read_in(path: &str) -> Vec<String> {
    let br = BufReader::new(std::fs::File::open(path).unwrap());
    br.lines()
        .filter_map(|line_result| {
            line_result.ok().and_then(|line| {
                if line.contains("d=\"") { Some(line) } else { None }
            })
        })
        .collect()
}

pub fn parse_svg(svg: Vec<&str>) -> Vec<Bezier> {
    //svg.iter().map(|l| {
    //    l.split(" ")
    //})
    vec!()
}
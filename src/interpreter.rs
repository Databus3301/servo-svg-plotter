use std::io::{BufRead, BufReader};
use crate::bezier::Bezier;

pub fn read_in(path: &str) -> Vec<&str> {
    let br = BufReader::new(std::fs::File::open(path).unwrap());
    br.lines().filter(|&l| *l.unwrap().contains("d=\"")).collect()
}

pub fn parse_svg(svg: Vec<&str>) -> Vec<Bezier> {
    svg.iter().map(|l| {
        l.split(" ")
    })
}
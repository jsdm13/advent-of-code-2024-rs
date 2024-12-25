use itertools::Itertools;
use regex::Regex;

fn main() {
    let pattern = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    let input = include_str!("../../day03a/full_input.txt")
        .split("don't()")
        .enumerate()
        .map(|(id, section)| {
            if id == 0 {
                return section.to_string();
            }
            section.split("do()").skip(1).join("")
        })
        .join("");

    let mut acc: u32 = 0;
    for (_, captures) in pattern.captures_iter(&input).map(|c| c.extract::<2>()) {
        acc += captures[0].parse::<u32>().unwrap() * captures[1].parse::<u32>().unwrap();
    }

    println!("acc: {acc}");
}

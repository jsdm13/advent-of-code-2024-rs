use regex::Regex;

fn main() {
    let pattern = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    let input = include_str!("../full_input.txt");

    let mut acc: u32 = 0;
    for (_, captures) in pattern.captures_iter(input).map(|c| c.extract::<2>()) {
        acc += captures[0].parse::<u32>().unwrap() * captures[1].parse::<u32>().unwrap();
    }

    println!("acc: {acc}");
}

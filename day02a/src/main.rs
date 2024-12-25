fn main() {
    let count = include_str!("../full_input.txt")
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(|report| {
            report.windows(2).all(|window| window[0] > window[1])
                || report.windows(2).all(|window| window[0] < window[1])
        })
        .filter(|report| {
            report.windows(2).all(|window| {
                (window[1] - window[0]).abs() <= 3 && (window[1] - window[0]).abs() > 0
            })
        })
        .count();

    println!("Count: {count}")
}

fn main() {
    println!(
        "Result: {}",
        include_str!("../../day02a/full_input.txt")
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|num| num.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
            })
            .map(|report| {
                let mut result: Vec<Vec<i32>> = Vec::new();
                result.push(report.clone());
                for i in 0..report.len() {
                    let mut mutation = report.clone();
                    mutation.remove(i);
                    result.push(mutation);
                }
                result
            })
            .map(|report_versions| {
                report_versions
                    .into_iter()
                    .filter(|report| {
                        report.windows(2).all(|window| window[0] > window[1])
                            || report.windows(2).all(|window| window[0] < window[1])
                    })
                    .collect::<Vec<Vec<i32>>>()
            })
            .map(|report_versions| {
                report_versions
                    .into_iter()
                    .filter(|report| {
                        report.windows(2).all(|window| {
                            (window[1] - window[0]).abs() <= 3 && (window[1] - window[0] != 0)
                        })
                    })
                    .collect::<Vec<Vec<i32>>>()
            })
            .filter(|report_versions| !report_versions.is_empty())
            .count()
    );
}

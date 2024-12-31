use std::collections::HashMap;

fn parse_rules(rules: &str) -> HashMap<usize, Vec<usize>> {
    let mut rule_map: HashMap<usize, Vec<usize>> = HashMap::new();

    for line in rules.lines() {
        let values: Vec<usize> = line.split('|').map(|val| val.parse().unwrap()).collect();

        rule_map
            .entry(values[0])
            .or_insert_with(Vec::new)
            .push(values[1]);
    }

    return rule_map;
}

fn main() {
    let input = include_str!("../full_input.txt");

    let mut sections = input.split("\r\n\r\n");

    let rules: HashMap<usize, Vec<usize>> = parse_rules(sections.next().unwrap());

    let sum = sections
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            line.split(',')
                .map(|id| id.parse::<usize>().unwrap())
                .rev()
                .collect::<Vec<_>>()
        })
        .filter(|line| {
            for i in 0..line.len() {
                if let Some(entry) = rules.get(&line[i]) {
                    for j in i..line.len() {
                        if entry.contains(&line[j]) {
                            return false;
                        }
                    }
                } else {
                    continue;
                }
            }
            true
        })
        .map(|valid_line| valid_line[valid_line.len() / 2])
        .sum::<usize>();

    println!("Total: {sum}");
}

use std::collections::{HashMap, VecDeque};

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

fn is_valid(line: &Vec<usize>, rules: &HashMap<usize, Vec<usize>>) -> bool {
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
}

fn make_valid(line: &Vec<usize>, rules: &HashMap<usize, Vec<usize>>) -> Vec<usize> {
    let mut indegree: HashMap<usize, usize> = HashMap::new();

    // Get the indegree of graph for topological sorting
    for &value in line {
        indegree.entry(value).or_insert(0);
        if let Some(entries) = rules.get(&value) {
            for entry in entries {
                if !line.contains(entry) {
                    continue;
                }
                indegree
                    .entry(*entry)
                    .and_modify(|val| *val += 1)
                    .or_insert(1);
            }
        }
    }

    // Use Kahn's algorithm for topologically sorting the local dependency graph
    // https://en.wikipedia.org/wiki/Topological_sorting
    let mut queue: VecDeque<usize> = indegree
        .iter()
        .filter(|(_, &val)| val == 0)
        .map(|(&key, _)| key)
        .collect();
    let mut result: Vec<usize> = Vec::with_capacity(line.len());

    while !queue.is_empty() {
        let current_page = queue.pop_front().expect("Queue should not be empty!");
        result.push(current_page);

        if let Some(neighbors) = rules.get(&current_page) {
            for &neighbor in neighbors.iter() {
                if !line.contains(&neighbor) {
                    continue;
                }
                indegree.entry(neighbor).and_modify(|value| *value -= 1);
                if *indegree.entry(neighbor).or_default() == 0 {
                    queue.push_back(neighbor);
                }
            }
        }
    }

    result
}

fn main() {
    let input = include_str!("../../day05a/full_input.txt");

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
        .filter(|line| !is_valid(line, &rules))
        .map(|line| make_valid(&line, &rules))
        .map(|valid_line| valid_line[valid_line.len() / 2])
        .sum::<usize>();

    println!("Total: {sum}");
}

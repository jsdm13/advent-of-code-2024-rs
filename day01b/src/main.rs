use std::collections::BTreeMap;

fn main() {
    let (list1, list2) = include_str!("../full_input.txt")
        .lines()
        .map(|line| {
            let test = line
                .split_whitespace()
                .filter_map(|num| num.parse::<i32>().ok())
                .collect::<Vec<_>>();
            test
        })
        .fold((Vec::<i32>::new(), Vec::<i32>::new()), |mut lists, pair| {
            lists.0.push(pair[0]);
            lists.1.push(pair[1]);
            lists
        });

    let count_map = list2
        .iter()
        .fold(BTreeMap::<i32, i32>::new(), |mut acc, i| {
            acc.entry(*i).and_modify(|val| *val += 1).or_insert(1);
            acc
        });

    let distance: i32 = list1
        .iter()
        .map(|val1| *val1 * count_map.get(val1).unwrap_or(&0))
        .sum();

    println!("Distance: {distance}")
}

fn main() {
    let (mut list1, mut list2) = include_str!("../full_input.txt")
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

    list1.sort_unstable();
    list2.sort_unstable();

    let distance: i32 = list1
        .iter()
        .zip(list2.iter())
        .map(|(val1, val2)| (val1 - val2).abs())
        .sum();

    println!("Distance: {distance}")
}

use std::collections::HashSet;
use std::sync::Arc;
use std::sync::RwLock;
struct Node {
    crossword: Arc<RwLock<Vec<Node>>>,
    letter: char,
    above: Option<usize>,
    below: Option<usize>,
    right: Option<usize>,
}

impl Node {
    fn contains_downward_diagonal(&self, text: &str) -> bool {
        if text.is_empty() {
            return true;
        }

        if self.letter != text.chars().next().unwrap() {
            return false;
        }

        if text.len() == 1 {
            return true;
        }

        if self.below.is_none() {
            return false;
        }

        let below_index = self.below.unwrap();

        let vec = self.crossword.read().unwrap();

        if vec[below_index].right.is_none() {
            return false;
        }

        let index = vec[below_index].right.unwrap();

        return vec[index].contains_downward_diagonal(&text[1..]);
    }

    fn contains_upward_diagonal(&self, text: &str) -> bool {
        if text.is_empty() {
            return true;
        }

        if self.letter != text.chars().next().unwrap() {
            return false;
        }

        if text.len() == 1 {
            return true;
        }

        if self.above.is_none() {
            return false;
        }

        let above_index = self.above.unwrap();

        let vec = self.crossword.read().unwrap();

        if vec[above_index].right.is_none() {
            return false;
        }

        let index = vec[above_index].right.unwrap();

        return vec[index].contains_upward_diagonal(&text[1..]);
    }
}

struct Crossword {
    nodes: Arc<RwLock<Vec<Node>>>,
}

impl Crossword {
    fn new(input: &str) -> Self {
        let lines = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<_>>();

        let size = lines[0].len();

        let nodes = Arc::new(RwLock::new(Vec::<Node>::with_capacity(size * size)));
        let mut node_writer = nodes.write().unwrap();

        for i in 0..size {
            for j in 0..size {
                let above = if i > 0 {
                    Some((i - 1) * size + j)
                } else {
                    None
                };

                let below = if i < (size - 1) {
                    Some((i + 1) * size + j)
                } else {
                    None
                };

                let right = if j < (size - 1) {
                    Some(i * size + j + 1)
                } else {
                    None
                };

                node_writer.push(Node {
                    letter: lines[i][j],
                    crossword: nodes.clone(),
                    above,
                    below,
                    right,
                });
            }
        }

        return Self {
            nodes: nodes.clone(),
        };
    }

    fn contains_downward_diagonal(&self, text: &str) -> HashSet<usize> {
        self.nodes
            .read()
            .unwrap()
            .iter()
            .enumerate()
            .map(|(idx, node)| (idx, node.contains_downward_diagonal(text)))
            .filter(|(_, contains)| *contains)
            .map(|(idx, _)| {
                let below = self.nodes.read().unwrap()[idx].below.unwrap();
                self.nodes.read().unwrap()[below].right.unwrap()
            })
            .collect()
    }
    fn contains_upward_diagonal(&self, text: &str) -> HashSet<usize> {
        self.nodes
            .read()
            .unwrap()
            .iter()
            .enumerate()
            .map(|(idx, node)| (idx, node.contains_upward_diagonal(text)))
            .filter(|(_, contains)| *contains)
            .map(|(idx, _)| {
                let above = self.nodes.read().unwrap()[idx].above.unwrap();
                self.nodes.read().unwrap()[above].right.unwrap()
            })
            .collect()
    }
}

fn main() {
    let input = include_str!("../../day04a/full_input.txt");

    let crossword = Crossword::new(input);

    // println!("DOWN:\t\tXMAS: {down_xmas}\tSAMX: {down_samx}");

    let mut downward_positions = crossword.contains_downward_diagonal("MAS");
    downward_positions.extend(crossword.contains_downward_diagonal("SAM"));

    // println!("DOWN DIAG:\tXMAS: {downward_diagonal_xmas}\tSAMX: {downward_diagonal_samx}");

    let mut upward_positions = crossword.contains_upward_diagonal("MAS");
    upward_positions.extend(crossword.contains_upward_diagonal("SAM"));

    let total = downward_positions.intersection(&upward_positions).count();

    // println!("UP DIAG:\tXMAS: {upward_diagonal_xmas}\tSAMX: {upward_diagonal_samx}");

    println!("Total matches: {total}");
}

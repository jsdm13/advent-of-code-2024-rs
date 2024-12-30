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
    fn contains_right(&self, text: &str) -> bool {
        if text.is_empty() {
            return true;
        }

        if self.letter != text.chars().next().unwrap() {
            return false;
        }

        if text.len() == 1 {
            return true;
        }

        if let Some(index) = self.right {
            // println!("{}", &text[1..]);
            let vec = self.crossword.read().unwrap();
            return vec[index].contains_right(&text[1..]);
        } else {
            return false;
        }
    }

    fn contains_down(&self, text: &str) -> bool {
        if text.is_empty() {
            return true;
        }

        if self.letter != text.chars().next().unwrap() {
            return false;
        }

        if text.len() == 1 {
            return true;
        }

        if let Some(index) = self.below {
            // println!("{}", &text[1..]);
            let vec = self.crossword.read().unwrap();
            return vec[index].contains_down(&text[1..]);
        } else {
            return false;
        }
    }

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

    fn contains_right(&self, text: &str) -> usize {
        self.nodes
            .read()
            .unwrap()
            .iter()
            .map(|node| node.contains_right(text))
            .filter(|contains| *contains)
            .count()
    }

    fn contains_down(&self, text: &str) -> usize {
        self.nodes
            .read()
            .unwrap()
            .iter()
            .map(|node| node.contains_down(text))
            .filter(|contains| *contains)
            .count()
    }
    fn contains_downward_diagonal(&self, text: &str) -> usize {
        self.nodes
            .read()
            .unwrap()
            .iter()
            .map(|node| node.contains_downward_diagonal(text))
            .filter(|contains| *contains)
            .count()
    }
    fn contains_upward_diagonal(&self, text: &str) -> usize {
        self.nodes
            .read()
            .unwrap()
            .iter()
            .map(|node| node.contains_upward_diagonal(text))
            .filter(|contains| *contains)
            .count()
    }
}

fn main() {
    let input = include_str!("../full_input.txt");

    let crossword = Crossword::new(input);

    let right_xmas = crossword.contains_right("XMAS");
    let right_samx = crossword.contains_right("SAMX");

    // println!("RIGHT:\t\tXMAS: {right_xmas}\tSAMX: {right_samx}");

    let down_xmas = crossword.contains_down("XMAS");
    let down_samx = crossword.contains_down("SAMX");

    // println!("DOWN:\t\tXMAS: {down_xmas}\tSAMX: {down_samx}");

    let downward_diagonal_xmas = crossword.contains_downward_diagonal("XMAS");
    let downward_diagonal_samx = crossword.contains_downward_diagonal("SAMX");

    // println!("DOWN DIAG:\tXMAS: {downward_diagonal_xmas}\tSAMX: {downward_diagonal_samx}");

    let upward_diagonal_xmas = crossword.contains_upward_diagonal("XMAS");
    let upward_diagonal_samx = crossword.contains_upward_diagonal("SAMX");

    // println!("UP DIAG:\tXMAS: {upward_diagonal_xmas}\tSAMX: {upward_diagonal_samx}");

    let total = right_xmas
        + right_samx
        + down_xmas
        + down_samx
        + downward_diagonal_xmas
        + downward_diagonal_samx
        + upward_diagonal_xmas
        + upward_diagonal_samx;

    println!("Total matches: {total}");
}

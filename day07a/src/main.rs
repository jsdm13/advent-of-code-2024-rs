struct Equation {
    sum: i64,
    expression: Vec<i64>,
}

fn evaluate(sum: i64, acc: i64, expression: &[i64]) -> bool {
    if expression.is_empty() {
        return false;
    }

    let tmp_add = acc + expression[0];
    let tmp_mul = acc * expression[0];

    if expression.len() == 1 {
        return tmp_add == sum || tmp_mul == sum;
    }

    evaluate(sum, tmp_add, &expression[1..]) || evaluate(sum, tmp_mul, &expression[1..])
}

fn main() {
    println!(
        "Result: {}",
        include_str!("../input.txt")
            .lines()
            .map(|line| {
                let (left, right) = line
                    .split_once(':')
                    .expect("Input line does not contain a colon!");

                let sum: i64 = left
                    .parse()
                    .expect("Left side of colon should evaluate to valid i32");
                let expression = right
                    .split_ascii_whitespace()
                    .map(|val| {
                        val.parse::<i64>().expect(
                            "Each value on the right side of the expression should be a valid i32",
                        )
                    })
                    .collect();

                Equation { sum, expression }
            })
            .map(|eq| {
                if evaluate(eq.sum, eq.expression[0], &eq.expression[1..]) {
                    eq.sum
                } else {
                    0i64
                }
            })
            .sum::<i64>()
    );
}

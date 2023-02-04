use mexprp;

pub fn solve(numbers: [u8; 4]) -> Vec<String> {
    let expressions = enumerate_expressions(numbers);

    // expressions
    //     .into_iter()
    //     .filter(|exp| match mexprp::eval::<f64>(exp) {
    //         Ok(mexprp::Answer::Single(24.0)) => true,
    //         _ => false,
    //     })
    //     .collect()

    expressions
        .into_iter()
        .filter(|exp| matches!(mexprp::eval::<f64>(exp), Ok(mexprp::Answer::Single(24.0))))
        .collect()
}

fn enumerate_expressions(numbers: [u8; 4]) -> Vec<String> {
    let number_combination = enumerate_numbers(numbers);
    let ops_combis = enumerate_operators();
    let mut result = Vec::new();

    number_combination.iter().for_each(|number_combi| {
        // parentheses patterns
        // (AB)(CD)
        ops_combis.iter().for_each(|ops_combi| {
            result.push(format!(
                "({}{}{}){}({}{}{})",
                number_combi[0],
                ops_combi[0],
                number_combi[1],
                ops_combi[1],
                number_combi[2],
                ops_combi[2],
                number_combi[3]
            ))
        });
        // A(BC)D
        ops_combis.iter().for_each(|ops_combi| {
            result.push(format!(
                "{}{}({}{}{}){}{}",
                number_combi[0],
                ops_combi[0],
                number_combi[1],
                ops_combi[1],
                number_combi[2],
                ops_combi[2],
                number_combi[3]
            ))
        });
        // ((AB)C)D
        ops_combis.iter().for_each(|ops_combi| {
            result.push(format!(
                "(({}{}{}){}{}){}{}",
                number_combi[0],
                ops_combi[0],
                number_combi[1],
                ops_combi[1],
                number_combi[2],
                ops_combi[2],
                number_combi[3]
            ))
        });
        // A((BC)D)
        ops_combis.iter().for_each(|ops_combi| {
            result.push(format!(
                "{}{}(({}{}{}){}{})",
                number_combi[0],
                ops_combi[0],
                number_combi[1],
                ops_combi[1],
                number_combi[2],
                ops_combi[2],
                number_combi[3]
            ))
        });
    });

    result
}

fn enumerate_numbers(numbers: [u8; 4]) -> Vec<[u8; 4]> {
    let mut result = Vec::new();

    for (pos1, a) in numbers.into_iter().enumerate() {
        for (pos2, b) in numbers.into_iter().enumerate() {
            if pos1 == pos2 {
                continue;
            }
            for (pos3, c) in numbers.into_iter().enumerate() {
                if pos3 == pos1 || pos3 == pos2 {
                    continue;
                }
                for (pos4, d) in numbers.into_iter().enumerate() {
                    if pos4 == pos1 || pos4 == pos2 || pos4 == pos3 {
                        continue;
                    }

                    result.push([a, b, c, d]);
                }
            }
        }
    }
    result
}

fn enumerate_operators() -> Vec<[char; 3]> {
    let operators = ['+', '-', '*', '/'];
    let mut result = Vec::new();
    operators.into_iter().for_each(|op1| {
        operators.into_iter().for_each(|op2| {
            operators.into_iter().for_each(|op3| {
                result.push([op1, op2, op3]);
            })
        })
    });

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enumerate_count() {
        let four_number_combinations = enumerate_numbers([1, 2, 3, 4]);
        assert_eq!(four_number_combinations.len(), 24);
    }

    #[test]
    fn enumerate_operator_count() {
        let operator_enumeration = enumerate_operators();
        assert_eq!(operator_enumeration.len(), 64);
    }
}

const INPUT: &str = include_str!("../data/input_3.txt");

const DO_OPS_TOKEN: &str = "do()";
const DONT_OPS_TOKEN: &str = "don't()";
const MUL_OPEN_BRACKET_TOKENS: &str = "mul(";
const MUL_CLOSE_BRACKET_TOKEN: char = ')';

fn scan_and_multiply(token_idx: usize, searching: &str) -> Option<i64> {
    let mul_seek_start = &searching[token_idx..];

    // check if we found the corresponding ')'
    let close_bracket_idx = mul_seek_start.find(MUL_CLOSE_BRACKET_TOKEN)?;

    let close_bracket_idx = close_bracket_idx + token_idx + 1;
    let mul_ops_string =
        &searching[(token_idx + MUL_OPEN_BRACKET_TOKENS.len())..(close_bracket_idx - 1)];

    for op_char in mul_ops_string.chars() {
        if !(op_char.is_numeric() || op_char == ',') {
            return None;
        }
    }

    // parse the two digits 11,12
    let (left_digit, right_digit) = mul_ops_string.split_once(',')?;
    Some(left_digit.parse::<i64>().unwrap() * right_digit.parse::<i64>().unwrap())
}

fn part_1() {
    let mut res = 0i64;
    let searching = INPUT.trim();

    let mut found_tokens = searching
        .match_indices(MUL_OPEN_BRACKET_TOKENS)
        .collect::<Vec<_>>();
    found_tokens.sort_by_key(|(idx, ..)| *idx);

    for (token_idx, ..) in &found_tokens {
        if let Some(scanned_res) = scan_and_multiply(*token_idx, searching) {
            res += scanned_res
        }
    }

    println!("part-1 res: {}", res);
}

fn part_2() {
    let mut do_mul = true;
    let mut res = 0i64;
    let searching = INPUT.trim();

    let mut found_tokens = searching.match_indices(DO_OPS_TOKEN).collect::<Vec<_>>();
    found_tokens.extend(searching.match_indices(DONT_OPS_TOKEN).collect::<Vec<_>>());
    found_tokens.extend(
        searching
            .match_indices(MUL_OPEN_BRACKET_TOKENS)
            .collect::<Vec<_>>(),
    );
    found_tokens.sort_by_key(|(idx, ..)| *idx);

    for (token_idx, token) in &found_tokens {
        match *token {
            DO_OPS_TOKEN => {
                do_mul = true;
            }
            DONT_OPS_TOKEN => {
                do_mul = false;
            }
            MUL_OPEN_BRACKET_TOKENS => {
                if let Some(scanned_res) = scan_and_multiply(*token_idx, searching) {
                    if do_mul {
                        res += scanned_res;
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    println!("part-2 res: {}", res);
}

fn main() {
    part_1();
    part_2();
}

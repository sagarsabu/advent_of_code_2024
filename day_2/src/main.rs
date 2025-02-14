const INPUT: &str = include_str!("../data/input_1.txt");

struct SafeRecordsAccumulatorPart1 {
    pub strictly_increasing: bool,
    pub strictly_decreasing: bool,
    pub zero_delta_detected: bool,
}

#[derive(Debug, Clone)]
struct SafeLevelsAccumulatorPart2 {
    pub strictly_increasing: bool,
    pub strictly_decreasing: bool,
    pub unsafe_delta_detected: bool,
    pub first_unsafe_level_idx: Option<usize>,
}

fn part_one() {
    let n_safe_records = INPUT.lines().fold(0, |acc, line| {
        // strictly increasing or decreasing in [1, 3] ranges
        let report = line
            .split_whitespace()
            .map(|record| record.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        let SafeRecordsAccumulatorPart1 {
            strictly_decreasing,
            strictly_increasing,
            zero_delta_detected,
        } = report.iter().enumerate().fold(
            SafeRecordsAccumulatorPart1 {
                strictly_decreasing: true,
                strictly_increasing: true,
                zero_delta_detected: false,
            },
            |mut acc, (idx, rec)| {
                if idx < report.len() - 1 {
                    let delta = rec - report[idx + 1];
                    acc.strictly_increasing &= delta >= 1 && delta <= 3;
                    acc.strictly_decreasing &= delta <= -1 && delta >= -3;
                    acc.zero_delta_detected |= delta == 0 || delta.abs() > 3;
                    acc
                } else {
                    // don't care about the last element
                    acc
                }
            },
        );

        let safe = (strictly_decreasing || strictly_increasing) && !zero_delta_detected;
        if safe {
            acc + 1
        } else {
            acc
        }
    });

    println!("part 1: n_safe_records {n_safe_records}");
}

fn part_two() {
    fn extract_safe_level_data(report: &Vec<i64>) -> SafeLevelsAccumulatorPart2 {
        let res = report.iter().enumerate().fold(
            SafeLevelsAccumulatorPart2 {
                strictly_decreasing: true,
                strictly_increasing: true,
                unsafe_delta_detected: false,
                first_unsafe_level_idx: None,
            },
            |mut acc, (idx, rec)| {
                if idx < report.len() - 1 {
                    let delta = rec - report[idx + 1];
                    acc.strictly_increasing &= delta >= 1 && delta <= 3;
                    acc.strictly_decreasing &= delta <= -1 && delta >= -3;
                    acc.unsafe_delta_detected |= delta == 0
                        || delta.abs() > 3
                        || (!acc.strictly_increasing && !acc.strictly_decreasing);

                    if acc.unsafe_delta_detected && acc.first_unsafe_level_idx.is_none() {
                        acc.first_unsafe_level_idx = Some(idx + 1);
                    }
                }

                acc
            },
        );
        res
    }

    let n_safe_records = INPUT.lines().fold(0, |acc, line| {
        // strictly increasing or decreasing in [1, 3] ranges
        let current_level = line
            .split_whitespace()
            .map(|record| record.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        let mut res = extract_safe_level_data(&current_level);
        let mut safe =
            (res.strictly_decreasing || res.strictly_increasing) && !res.unsafe_delta_detected;

        if !safe {
            // all permutations with one element removed
            for idx in 0..current_level.len() {
                let mut level_perm = current_level.clone();
                level_perm.remove(idx);
                res = extract_safe_level_data(&level_perm);
                safe = (res.strictly_decreasing || res.strictly_increasing)
                    && !res.unsafe_delta_detected;

                if safe {
                    break;
                }
            }
        }

        if safe {
            acc + 1
        } else {
            acc
        }
    });

    println!("part 2: n_safe_records {n_safe_records}");
}

fn main() {
    part_one();
    part_two();
}

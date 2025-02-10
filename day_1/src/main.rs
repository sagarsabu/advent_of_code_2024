const INPUT: &str = include_str!("../data/input_1.txt");

fn part_one() {
    let (mut left, mut right) =
        INPUT
            .lines()
            .fold((vec![], vec![]), |(mut left_acc, mut right_acc), line| {
                let mut separated_line = line.split_whitespace();
                let left = separated_line.next().unwrap().parse::<i64>().unwrap();
                let right = separated_line.next().unwrap().parse::<i64>().unwrap();

                left_acc.push(left);
                right_acc.push(right);

                (left_acc, right_acc)
            });

    left.sort();
    right.sort();

    assert_eq!(left.len(), right.len());

    let distance = left
        .iter()
        .zip(right)
        .fold(0i64, |acc, (a, b)| acc + (a - b).abs());

    println!("distance {}", distance);
}

fn part_two() {
    let (left, right) =
        INPUT
            .lines()
            .fold((vec![], vec![]), |(mut left_acc, mut right_acc), line| {
                let mut separated_line = line.split_whitespace();
                let left = separated_line.next().unwrap().parse::<i64>().unwrap();
                let right = separated_line.next().unwrap().parse::<i64>().unwrap();

                left_acc.push(left);
                right_acc.push(right);

                (left_acc, right_acc)
            });

    let total_similar = left.into_iter().fold(0, |acc, cur| {
        let similarities = right.iter().filter(|x| **x == cur).count() as i64;
        acc + (cur * similarities)
    });

    println!("total_similar {}", total_similar);
}

fn main() {
    part_one();
    part_two();
}

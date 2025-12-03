use std::{
    cmp::Reverse,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone, Copy, PartialEq)]
struct NumberWithPosition {
    number: u64,
    position: usize,
}

pub fn part1() -> u64 {
    let file = File::open("./inputs/03").unwrap();
    let reader = BufReader::new(file);
    let mut somma = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        let mut numbers: Vec<NumberWithPosition> = line
            .chars()
            .enumerate()
            .map(|(i, x)| NumberWithPosition {
                number: x.to_digit(10).unwrap() as u64,
                position: i,
            })
            .collect();

        numbers.sort_by_key(|x| Reverse(x.number));

        let mut first = numbers[0];
        let mut second = NumberWithPosition {
            number: 0,
            position: 0,
        };

        for i in 0..numbers.len() {
            for num in &numbers {
                if num.position > first.position {
                    second = *num;
                    break;
                }
            }
            if second.number != 0 {
                break;
            }
            first = numbers[i + 1];
        }

        somma += first.number * 10 + second.number;
    }

    somma
}

pub fn part2() -> u64 {
    let file = File::open("./inputs/03").unwrap();
    let reader = BufReader::new(file);
    let mut somma = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        let numbers: Vec<u64> = line
            .chars()
            .map(|x| x.to_digit(10).unwrap() as u64)
            .collect();

        let mut collected: Vec<u64> = vec![];
        let mut last_collected_index = 0;
        let mut tmp: u64 = 0;

        for i in (0..12).rev() {
            for j in last_collected_index..(numbers.len() - i) {
                if numbers[j] > tmp {
                    tmp = numbers[j];
                    last_collected_index = j + 1;
                }
            }
            collected.push(tmp);
            tmp = 0;
        }

        somma += collected.iter().fold(0u64, |acc, x| acc * 10 + x);
    }

    somma
}

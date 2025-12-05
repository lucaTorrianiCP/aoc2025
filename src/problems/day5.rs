use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct Range {
    first: u64,
    last: u64,
}

pub fn part1() -> u64 {
    let file = File::open("./inputs/05").unwrap();
    let reader = BufReader::new(file);
    let mut somma = 0;
    let mut ranges: Vec<Range> = vec![];
    let mut numbers: Vec<u64> = vec![];

    for line in reader.lines() {
        let line = line.unwrap();

        match line.as_str() {
            "" => continue,
            s if s.contains("-") => {
                let (first, last) = s.split_once("-").unwrap();
                ranges.push(Range {
                    first: first.parse().unwrap(),
                    last: last.parse().unwrap(),
                });
            }
            s => {
                numbers.push(s.parse().unwrap());
            }
        }
    }

    for num in numbers {
        for range in &ranges {
            if (range.first..=range.last).contains(&num) {
                somma += 1;
                break;
            }
        }
    }

    somma
}

pub fn part2() -> u64 {
    let file = File::open("./inputs/05").unwrap();
    let reader = BufReader::new(file);

    let mut ranges: Vec<Range> = reader
        .lines()
        .map(|l| l.unwrap())
        .filter(|s| s.contains('-'))
        .map(|s| {
            let (first, last) = s.split_once('-').unwrap();
            Range {
                first: first.parse().unwrap(),
                last: last.parse().unwrap(),
            }
        })
        .collect();

    ranges.sort_by_key(|x| x.first);

    let mut merged: Vec<Range> = Vec::new();

    for range in ranges {
        if let Some(last) = merged.last_mut() {
            if range.first <= last.last + 1 {
                last.last = last.last.max(range.last);
                continue;
            }
        }
        merged.push(Range {
            first: range.first,
            last: range.last,
        });
    }

    merged.iter().map(|x| x.last - x.first + 1).sum()
}

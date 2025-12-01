use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

#[derive(Debug)]
struct Movement {
    side: char,
    value: i32,
}

#[derive(Debug)]
struct ParseMovementError {}

impl FromStr for Movement {
    type Err = ParseMovementError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (side, value) = s.split_at(1);

        let value = value.parse::<i32>().map_err(|_| ParseMovementError {})?;

        Ok(Movement {
            side: side.chars().next().unwrap(),
            value,
        })
    }
}

trait AddWithLimit {
    fn add_with_limit(&mut self, amount: i32) -> i32;
    fn sub_with_limit(&mut self, amount: i32) -> i32;
}

impl AddWithLimit for i32 {
    fn add_with_limit(&mut self, amount: i32) -> i32 {
        let total = *self + amount;
        let overflows = total.div_euclid(100);
        *self = total.rem_euclid(100);
        overflows
    }

    fn sub_with_limit(&mut self, amount: i32) -> i32 {
        let initial_self = *self;
        let total = *self - amount;
        let mut underflows = total.div_euclid(100).abs();
        *self = total.rem_euclid(100);

        if *self == 0 {
            underflows = underflows.abs() + 1;
        }

        if initial_self == 0 {
            underflows = underflows.abs() - 1;
        }

        underflows
    }
}

pub fn part2() -> i32 {
    let file = File::open("./inputs/01").unwrap();
    let reader = BufReader::new(file);
    let mut dial: i32 = 50;
    let mut counter = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        let movement = line.parse::<Movement>().unwrap();

        counter += match movement.side {
            'L' => dial.sub_with_limit(movement.value),
            'R' => dial.add_with_limit(movement.value),
            _ => {
                panic!()
            }
        };
    }

    counter
}

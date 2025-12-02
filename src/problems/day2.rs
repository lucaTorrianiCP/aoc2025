use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn part1() -> u64 {
    let file = File::open("./inputs/02").unwrap();
    let reader = BufReader::new(file);
    let line = reader.lines().next().unwrap().unwrap();
    let mut somma: u64 = 0;

    for range in line.split(",") {
        let mut split = range.split("-");
        let from = split.next().unwrap().to_string().parse::<u64>().unwrap();
        let to = split.next().unwrap().to_string().parse::<u64>().unwrap();

        somma += (from..=to)
            .filter(|x| {
                let cifre = x.ilog10() + 1;

                if cifre % 2 == 1 {
                    return false;
                }

                let half_point = cifre / 2;

                let first_half = x % 10_u64.pow(half_point);
                let second_half = x / 10_u64.pow(half_point);

                if first_half == second_half {
                    return true;
                }

                false
            })
            .sum::<u64>();
    }

    somma
}

pub fn part2() -> u64 {
    let file = File::open("./inputs/02").unwrap();
    let reader = BufReader::new(file);
    let line = reader.lines().next().unwrap().unwrap();
    let mut somma: u64 = 0;

    for range in line.split(",") {
        let mut split = range.split("-");

        let from = split.next().unwrap().to_string().parse::<u64>().unwrap();
        let to = split.next().unwrap().to_string().parse::<u64>().unwrap();

        somma += (from..=to)
            .filter(|x| {
                let cifre = x.ilog10() + 1;

                for i in 1..=cifre / 2 {
                    if cifre % i == 1 {
                        continue;
                    }
                    let ripetizioni = cifre / i;
                    let part = x / 10_u64.pow((ripetizioni - 1) * i);
                    let numeratore = 10_u64.pow(ripetizioni * i) - 1;
                    let denom = 10_u64.pow(i) - 1;
                    let numero_ricostruito = part * (numeratore / denom);
                    if numero_ricostruito == *x {
                        return true;
                    }
                }

                false
            })
            .sum::<u64>();
    }

    somma
}

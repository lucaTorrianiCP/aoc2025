use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn part1() -> u64 {
    let file = File::open("./inputs/07").unwrap();
    let reader = BufReader::new(file);
    let mut somma = 0;

    let mut matrix: Vec<Vec<char>> = reader
        .lines()
        .map(|x| x.unwrap().chars().collect())
        .collect();

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == 'S' {
                matrix[i + 1][j] = '|';
            }
            if matrix[i][j] == '|' && i + 1 < matrix.len() {
                if matrix[i + 1][j] == '^' {
                    somma += 1;
                    matrix[i + 1][j - 1] = '|';
                    matrix[i + 1][j + 1] = '|';
                } else if i < matrix.len() {
                    matrix[i + 1][j] = '|';
                }
            }
        }
    }

    somma
}

pub fn part2() -> u64 {
    let file = File::open("./inputs/07").unwrap();
    let reader = BufReader::new(file);
    let somma;

    let matrix: Vec<Vec<char>> = reader
        .lines()
        .filter(|x| x.as_ref().unwrap().contains("^") || x.as_ref().unwrap().contains("S"))
        .map(|x| x.unwrap().chars().collect())
        .collect();

    let mut indexes: Vec<u64> = vec![0; matrix[0].len()];

    for i in 0..matrix[0].len() {
        if matrix[0][i] == 'S' {
            indexes[i] = 1;
        }
    }

    for i in 0..matrix.len() {
        let mut next_indexs = vec![0; matrix[i].len()];
        for j in 0..matrix[i].len() {
            if matrix[i][j] == '^' {
                next_indexs[j - 1] += indexes[j];
                next_indexs[j + 1] += indexes[j];
            } else {
                next_indexs[j] += indexes[j];
            }
        }
        indexes = next_indexs;
    }

    somma = indexes.iter().sum::<u64>();

    somma
}

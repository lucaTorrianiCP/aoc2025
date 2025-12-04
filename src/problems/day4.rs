use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn count_neighbors(grid: &Vec<Vec<char>>, row: usize, col: usize) -> usize {
    let neighbors = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    neighbors
        .iter()
        .filter_map(|(delta_row, delta_col)| {
            let neighbor_row = row.checked_add_signed(*delta_row)?;
            let neighbor_col = col.checked_add_signed(*delta_col)?;
            grid.get(neighbor_row)?.get(neighbor_col)
        })
        .filter(|&&pos| pos == '@')
        .count()
}

pub fn part1() -> u64 {
    let file = File::open("./inputs/04").unwrap();
    let reader = BufReader::new(file);
    let mut somma = 0;

    let matrix: Vec<Vec<char>> = reader
        .lines()
        .map(|x| x.unwrap().chars().collect::<Vec<char>>())
        .collect();

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            match matrix[i][j] {
                '@' => {
                    if count_neighbors(&matrix, i, j) < 4 {
                        somma += 1;
                    }
                }
                _ => continue,
            }
        }
    }

    somma
}

pub fn part2() -> u64 {
    let file = File::open("./inputs/04").unwrap();
    let reader = BufReader::new(file);
    let mut removed;
    let mut somma = 0;

    let mut matrix: Vec<Vec<char>> = reader
        .lines()
        .map(|x| x.unwrap().chars().collect::<Vec<char>>())
        .collect();

    loop {
        let mut new_matrix = matrix.clone();
        removed = 0;
        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                match matrix[i][j] {
                    '@' => {
                        if count_neighbors(&matrix, i, j) < 4 {
                            removed += 1;
                            new_matrix[i][j] = '.';
                        }
                    }
                    _ => continue,
                }
            }
        }
        if removed == 0 {
            break;
        }

        somma += removed;
        matrix = new_matrix;
    }

    somma
}

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn part1() -> u64 {
    let file = File::open("./inputs/06").unwrap();
    let reader = BufReader::new(file);
    let mut somma: u64 = 0;

    let mut matrix: Vec<Vec<String>> = reader
        .lines()
        .map(|x| {
            x.unwrap()
                .split_whitespace()
                .map(|y| y.to_string())
                .collect()
        })
        .collect();

    let operations = matrix.pop().unwrap();

    for i in 0..operations.len() {
        let mut partial_res: u64 = 0;
        let operation = operations[i].clone();
        for row in &matrix {
            match operation.as_str() {
                "*" => {
                    if partial_res == 0 {
                        partial_res = 1;
                    }
                    partial_res *= row[i].parse::<u64>().unwrap();
                }
                "+" => {
                    partial_res += row[i].parse::<u64>().unwrap();
                }
                _ => panic!(),
            }
        }

        somma += partial_res;
    }

    somma
}

pub fn part2() -> u64 {
    let file = File::open("./inputs/06").unwrap();
    let reader = BufReader::new(file);
    let mut somma: u64 = 0;

    let matrix: Vec<Vec<String>> = reader
        .lines()
        .map(|x| x.unwrap().split(" ").map(|y| y.to_string()).collect())
        .collect();

    let mut matrix = align_matrix(matrix);

    let operations: Vec<String> = matrix
        .pop()
        .unwrap()
        .iter()
        .map(|x| x.trim().to_string())
        .collect();

    let mut new_matrix: Vec<Vec<String>> = vec![];

    for i in 0..operations.len() {
        let mut tmp_vec: Vec<String> = vec![];
        for row in &matrix {
            tmp_vec.push(row[i].clone());
        }
        new_matrix.push(tmp_vec);
    }

    let mut new_matrix_nums: Vec<Vec<u64>> = vec![];

    for vec in new_matrix {
        let mut matrix_chars: Vec<Vec<char>> = vec![vec![]; vec.len()];
        for i in 0..vec.len() {
            let chars: Vec<char> = vec[i].chars().into_iter().collect();

            for j in 0..chars.len() {
                matrix_chars[j].push(chars[j]);
            }
        }
        let nums_vec: Vec<u64> = matrix_chars
            .into_iter()
            .map(|chars| chars.into_iter().collect::<String>())
            .filter(|y| !y.is_empty())
            .map(|x| x.trim().parse::<u64>().unwrap())
            .collect();

        new_matrix_nums.push(nums_vec);
    }

    for i in 0..operations.len() {
        match operations[i].as_str() {
            "*" => {
                somma += new_matrix_nums[i].iter().fold(1u64, |acc, v| acc * v);
            }
            "+" => {
                somma += new_matrix_nums[i].iter().fold(0u64, |acc, v| acc + v);
            }
            _ => {}
        }
    }

    somma
}

fn align_matrix(matrix: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let cleaned: Vec<Vec<&String>> = matrix
        .iter()
        .map(|row| row.iter().filter(|s| !s.is_empty()).collect())
        .collect();

    let col_count = cleaned.iter().map(|row| row.len()).max().unwrap_or(0);
    let mut max_lens = vec![0; col_count];
    for row in &cleaned {
        for (i, val) in row.iter().enumerate() {
            max_lens[i] = max_lens[i].max(val.len());
        }
    }

    let mut result = vec![];

    for i in 0..matrix.len() {
        let riga = matrix[i].clone();
        let mut new_riga: Vec<String> = vec![];
        let mut tmp_string = String::new();
        let mut lens_index = 0;

        for j in 0..riga.len() {
            if riga[j] == "".to_string() {
                tmp_string.push_str(" ");
            } else {
                tmp_string.push_str(riga[j].as_str());
            }

            if tmp_string.len() == max_lens[lens_index] {
                new_riga.push(tmp_string);
                tmp_string = String::new();
                lens_index += 1;
            }
        }

        result.push(new_riga);
    }

    result
}

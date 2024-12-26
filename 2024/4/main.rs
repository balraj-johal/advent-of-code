use std::{collections::HashMap, fs};

#[derive(Debug)]
struct Letter {
    letter: String,
    column_index: i32,
    row_index: i32,
}

fn get_letter_key(column_index: i32, row_index: i32) -> String {
    return format!("{row_index}-{column_index}");
}

struct ProcessedLetters {
    letters: HashMap<String, Letter>,
    num_columns: i32,
    num_rows: i32,
}

const REQUIRED_LETTERS: [&str; 3] = ["M", "A", "S"];

fn get_processed_letters() -> ProcessedLetters {
    let input_raw: String =
        fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let input = input_raw.as_str();
    let lines = input.lines();
    let num_rows: i32 = i32::try_from(input.lines().count()).unwrap();

    let mut processed_letters: HashMap<String, Letter> = HashMap::new();
    let mut row_count: i32 = 0;

    let mut num_columns: i32 = 0;

    for line in lines {
        num_columns = i32::try_from(line.len()).unwrap();

        let mut column_count: i32 = 0;
        let letters = line.split("");
        for letter in letters {
            if letter == "" {
                continue;
            }

            let letter_struct = Letter {
                letter: letter.to_string(),
                column_index: column_count,
                row_index: row_count,
            };

            let key = get_letter_key(column_count, row_count);
            processed_letters.insert(key, letter_struct);

            column_count += 1;
        }

        row_count += 1;
    }

    return ProcessedLetters {
        letters: processed_letters,
        num_columns: num_columns,
        num_rows: num_rows,
    };
}

fn does_letter_match(
    letters: &HashMap<String, Letter>,
    column: i32,
    row: i32,
    required_letter: &str,
) -> bool {
    let key: String = get_letter_key(column, row);
    match letters.get(&key) {
        Some(value) => {
            return value.letter == required_letter;
        }
        None => {
            return false;
        }
    }
}

fn generic_matches(
    letters: &HashMap<String, Letter>,
    target: &Letter,
    row_offsets: Vec<i32>,
    column_offsets: Vec<i32>,
) -> i32 {
    let mut does_match: bool = true;

    for index in 0..3 {
        let row_offset = row_offsets[index];
        let column_offset = column_offsets[index];
        let required_letter = REQUIRED_LETTERS[index];

        if does_match == false {
            break;
        }

        does_match = does_letter_match(
            letters,
            target.column_index - column_offset,
            target.row_index - row_offset,
            required_letter,
        );
    }

    if does_match {
        return 1;
    }

    return 0;
}

fn matches(letters: &HashMap<String, Letter>, target: &Letter) -> i32 {
    let mut total_matches: i32 = 0;

    // horizontal forward backward
    total_matches += generic_matches(&letters, target, vec![0, 0, 0], vec![1, 2, 3]);
    total_matches += generic_matches(&letters, target, vec![0, 0, 0], vec![-1, -2, -3]);

    // vertical forward backward
    total_matches += generic_matches(&letters, target, vec![1, 2, 3], vec![0, 0, 0]);
    total_matches += generic_matches(&letters, target, vec![-1, -2, -3], vec![0, 0, 0]);

    // diagonal forward up/down
    total_matches += generic_matches(&letters, target, vec![1, 2, 3], vec![1, 2, 3]);
    total_matches += generic_matches(&letters, target, vec![-1, -2, -3], vec![1, 2, 3]);

    // diagonal backwards up/down
    total_matches += generic_matches(&letters, target, vec![1, 2, 3], vec![-1, -2, -3]);
    total_matches += generic_matches(&letters, target, vec![-1, -2, -3], vec![-1, -2, -3]);

    return total_matches;
}

fn main() {
    let processed = get_processed_letters();
    let letters: HashMap<String, Letter> = processed.letters;
    let mut num_matches: i32 = 0;

    for row_index in 0..processed.num_rows {
        for column_index in 0..processed.num_columns {
            let key = get_letter_key(column_index, row_index);
            let letter = letters.get(&key).unwrap();

            if letter.letter != "X" {
                continue;
            }

            num_matches += matches(&letters, letter);
        }
    }

    println!("{}", num_matches);
}

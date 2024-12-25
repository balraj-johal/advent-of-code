use std::{collections::HashMap, fs};

#[derive(Debug)]
struct Letter {
    letter: String,
    flat_index: i32,
    column_index: i32,
    row_index: i32,
}

fn get_letter_key(column_index: i32, row_index: i32) -> String {
    return format!("{row_index}-{column_index}");
}

fn get_processed_letters() -> HashMap<String, Letter> {
    let input_raw: String =
        fs::read_to_string("test.txt").expect("Should have been able to read the file");
    let input = input_raw.as_str();
    let lines = input.lines();

    let mut processed_letters: HashMap<String, Letter> = HashMap::new();
    let mut flat_count: i32 = 0;
    let mut row_count: i32 = 0;

    for line in lines {
        let mut column_count: i32 = 0;
        let letters = line.split("");
        for letter in letters {
            if letter == "" {
                continue;
            }

            let letter_struct = Letter {
                letter: letter.to_string(),
                flat_index: flat_count,
                column_index: column_count,
                row_index: row_count,
            };

            let key = get_letter_key(column_count, row_count);
            processed_letters.insert(key, letter_struct);

            column_count += 1;
            flat_count += 1;
        }

        row_count += 1;
    }

    return processed_letters;
}

fn main() {
    let mut letters: HashMap<String, Letter> = get_processed_letters();

    for letter in letters {
        println!("{:?}", letter);
    }
}

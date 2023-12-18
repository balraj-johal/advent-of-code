use std::fs;
use std::collections::HashMap;

const ID_START_OFFSET: usize = 5;

fn get_valid_state() -> HashMap<String, i16> {
    let mut valid_state = HashMap::new();
    
    valid_state.insert("red".to_string(), 12);
    valid_state.insert("green".to_string(), 13);
    valid_state.insert("blue".to_string(), 14);

    return valid_state;
}

fn get_substring(game: &String, start: usize, end: usize) -> &str {
    return &game[start..end];
}

fn get_game_id(game: &String) -> i16 {
    let end_of_id_index = game.find(":").unwrap();
    let id_substring = get_substring(&game, ID_START_OFFSET, end_of_id_index);

    // parse substring to i16
    return id_substring.parse::<i16>().unwrap();
}

fn get_game_state_string(game: &String) -> &str {
    let end_of_id_index = game.find(":").unwrap();
    let state_string = get_substring(&game, end_of_id_index + 2, game.len());

    // parse substring to i16
    return state_string;
}

fn check_game_validity(game: String, valid_state: &HashMap<String, i16>) -> bool {
    let reveals = game.split(";");
    let mut valid = true;
    'outer: for reveal in reveals {
        let colors = reveal.split(",");
        for color in colors {
            let mut safe_color = color;
            // remove leading spaces
            if safe_color[0..1].to_string() == " " {
                safe_color = &safe_color[1..safe_color.len()];
            };
            let mut value_key_pair = safe_color.split(" ");
            let value = value_key_pair.next().unwrap().parse::<i16>().unwrap();
            let key = value_key_pair.next().unwrap();

            let allowed_value = valid_state[key];
            if value > allowed_value {
                valid = false;
                break 'outer;
            }
        }
    }
    return valid;
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");
    let games = input.lines();
    let valid_state = get_valid_state();

    let mut valid_game_ids: Vec<i16> = Vec::new();
    let mut sum_of_ids: i16 = 0;

    for game in games {
        let game_string = game.to_string();
        let game_state_string = get_game_state_string(&game_string);
        let game_valid = check_game_validity(game_state_string.to_string(), &valid_state);

        if game_valid {
            let game_id = get_game_id(&game_string);
            valid_game_ids.push(game_id);
        }
    }
    
    for id in valid_game_ids {
        sum_of_ids = &sum_of_ids + id;
    }
    
    println!("{}", sum_of_ids);
}

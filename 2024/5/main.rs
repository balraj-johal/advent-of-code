use std::fs;

struct ProcessedInput {
    rules: Vec<[i32; 2]>,
    pages: Vec<Vec<i32>>,
}

fn parse_input() -> ProcessedInput {
    let input_raw: String =
        fs::read_to_string("test.txt").expect("Should have been able to read the file");
    let input = input_raw.as_str();
    let lines = input.lines();

    let mut rules: Vec<[i32; 2]> = vec![];
    let mut pages: Vec<Vec<i32>> = vec![];

    for line in lines {
        if line.contains("|") {
            let mut split = line.split("|").map(|value| value.parse::<i32>().unwrap());
            let first = split.next().unwrap();
            let second = split.next().unwrap();
            rules.push([first, second]);
        }

        if line.contains(",") {
            let mut split = line.split(",").map(|value| value.parse::<i32>().unwrap());
            let page: Vec<i32> = split.collect();
            pages.push(page);
        }
    }
    println!("rules {:?}", rules);
    println!("pages {:?}", pages);

    return ProcessedInput { rules, pages };
}

fn main() {
    let processed = parse_input();
    let mut total: i32 = 0;

    println!("result: {}", total);
}

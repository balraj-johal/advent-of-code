use std::fs;

fn main() {
    let loaded_input: String =
        fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let mut reports: Vec<Vec<i32>> = Vec::new();
    let mut unsafe_report_count = 0;

    let lines = loaded_input.lines();

    for line in lines {
        let strings = line.split_whitespace();
        let mut numbers: Vec<i32> = Vec::new();

        for string in strings {
            let number: i32 = string
                .parse()
                .expect("Should have parsed number from string");

            numbers.push(number);
        }

        reports.push(numbers);
    }

    let number_of_reports = reports.len();

    for report in reports {
        let mut ascending = false;
        let mut descending = false;

        let length: usize = report.len();
        let interations: usize = length - 1;

        for n in 0..interations {
            let current_value = report[n];
            let next_value = report[n + 1];

            let diff = next_value - current_value;
            if diff > 0 {
                ascending = true;
            } else {
                descending = true;
            }

            if descending && ascending {
                unsafe_report_count += 1;
                break;
            }

            let absolute_diff = diff.abs();

            if absolute_diff < 1 || absolute_diff > 3 {
                unsafe_report_count += 1;
                break;
            }
        }
    }

    println!("{:#?}", number_of_reports - unsafe_report_count);
}

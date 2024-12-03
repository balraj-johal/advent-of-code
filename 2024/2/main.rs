use std::fs;

fn check_pair(ascending: &mut bool, descending: &mut bool, current: &i32, next: &i32) -> bool {
    let diff = next - current;
    if diff > 0 {
        *ascending = true;
    } else {
        *descending = true;
    }

    if *descending && *ascending {
        println!("pair {:?}, {:?} is not safe due to order", &current, &next);
        return true;
    }

    let absolute_diff: i32 = diff.abs();

    if absolute_diff < 1 || absolute_diff > 3 {
        println!(
            "pair {:?}, {:?} is not safe due to diff of {:?}",
            &current, &next, absolute_diff
        );
        return true;
    }

    return false;
}

fn check_report_safety(report: &Vec<i32>, override_has_tolerance: bool) -> bool {
    let mut ascending: bool = false;
    let mut descending: bool = false;
    let mut is_unsafe: bool = false;
    let mut has_tolerance: bool = override_has_tolerance.clone();

    let length: usize = report.len();
    let mut index: usize = 0;

    while index < length - 1 {
        let current: i32 = report[index];
        let next: i32 = report[index + 1];

        let cloned_ascending: bool = ascending.clone();
        let cloned_descending: bool = descending.clone();

        let pair_safe: bool = !check_pair(&mut ascending, &mut descending, &current, &next);

        if pair_safe {
            println!("pair {:?}, {:?} is safe", &current, &next);
            index += 1;
            continue;
        }

        if !has_tolerance {
            is_unsafe = true;
            println!(
                "pair {:?}, {:?} is not safe, no tolerance left",
                &current, &next
            );

            index += 1;
            continue;
        }

        if index == 0 {
            let new_current: &i32 = &report[index + 1];
            let new_next: &i32 = &report[index + 2];

            ascending = cloned_ascending;
            descending = cloned_descending;

            let next_pair_safe: bool =
                !check_pair(&mut ascending, &mut descending, &new_current, &new_next);

            if next_pair_safe {
                println!(
                    "pair {:?}, {:?} is safe, no more tolerance",
                    &current, &new_next
                );
                has_tolerance = false;
                is_unsafe = false;

                index += 1;
                continue;
            } else {
                println!(
                    "pair {:?}, {:?} is not safe after skip",
                    &current, &new_next
                );
                is_unsafe = true;
            }
        }

        ascending = cloned_ascending;
        descending = cloned_descending;

        if index + 2 < length {
            let new_next: &i32 = &report[index + 2];
            let next_pair_safe: bool =
                !check_pair(&mut ascending, &mut descending, &current, &new_next);

            if next_pair_safe {
                println!(
                    "pair {:?}, {:?} is safe, no more tolerance",
                    &current, &new_next
                );
                has_tolerance = false;
                is_unsafe = false;
            } else {
                println!(
                    "pair {:?}, {:?} is not safe after skip",
                    &current, &new_next
                );
                is_unsafe = true;
            }

            index += 1;
        }

        index += 1;
    }

    println!("{:?}", report);
    println!("{:?}", !is_unsafe);

    return is_unsafe;
}

fn main() {
    let loaded_input: String =
        fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let mut reports: Vec<Vec<i32>> = Vec::new();
    let mut unsafe_report_count: usize = 0;

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

    let number_of_reports: usize = reports.len();

    for report in reports {
        if check_report_safety(&report, true) {
            unsafe_report_count += 1;
        }
    }

    println!("{:#?}", number_of_reports - unsafe_report_count);
}

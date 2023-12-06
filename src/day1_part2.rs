use std::{collections::HashMap, fs};

fn main() {
    let digits = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);
    let text =
        fs::read_to_string("/home/dev/adventofcode/src/day1.txt").expect("Fail reading file");
    let mut total = 0;
    for line in text.lines() {
        println!("content:\n{:?}", line);
        let mut found_str = String::new();
        let mut found_digits = vec![];
        for (t_num, num) in &digits {
            let mut f: Vec<_> = line.match_indices(t_num).collect();
            let mut e: Vec<_> = line.match_indices(num).collect();

            if !f.is_empty() {
                found_digits.append(&mut f);
            }
            if !e.is_empty() {
                found_digits.append(&mut e);
            }
        }
        found_digits.sort();
        if !found_digits.is_empty() {
            let first = found_digits.first().unwrap();
            let last = found_digits.last().unwrap();
            if first.1.chars().next().unwrap().is_numeric() {
                found_str.push_str(first.1);
            } else {
                let d = digits.get(first.1).unwrap();
                found_str.push_str(d);
            }
            if last.1.chars().next().unwrap().is_numeric() {
                found_str.push_str(last.1);
            } else {
                let d = digits.get(last.1).unwrap();
                found_str.push_str(d);
            }
        }

        println!("{:?}, Found didgits", found_digits);
        println!("{:?}, Found string", found_str);
        let found_num: i32 = found_str.parse().unwrap();
        println!("{:?}, Found number", found_num);

        total = total + found_num;
        println!("{:?}, Total", total);
    }
}

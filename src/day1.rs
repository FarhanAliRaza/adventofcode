use std::fs;

fn main() {
    let text =
        fs::read_to_string("/home/dev/adventofcode/src/day1.txt").expect("Fail reading file");

    let mut total = 0;
    for line in text.lines() {
        println!("content:\n{:?}", line);
        let mut line_num = String::new();
        for ch in line.chars() {
            // println!("{ch}");

            if ch.is_numeric() {
                line_num.push_str(ch.to_string().as_str());
            }
        }
        println!("Number Found String: {line_num}");
        let mut number = 0;
        if (line_num.len() == 0) {
            continue;
        } else {
            let first_char = line_num.chars().nth(0).unwrap();
            let last_char = line_num.chars().last().unwrap();
            number = format!("{first_char}{last_char}").parse().unwrap();
        } //71878
        println!("Number Found Int: {number}");

        total = total + number;
        println!("Total: {total}");
    }
}

use regex::Regex;
use std::fs::read_to_string;
use std::string::String;

fn get_input() -> String {
    let path = "inputs/day_3.txt";
    read_to_string(path).expect("File was not found")
}

fn parse(data: String) -> Vec<String> {
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").expect("Expression creation failed");

    let mut entries: Vec<String> = Vec::new();

    for mat in re.find_iter(data.as_str()) {
        let str: String = data[mat.start()..mat.end()].to_string();
        entries.push(str);
    }

    entries
}

fn parse_with_instructions(data: String) -> Vec<String> {
    let mut entries: Vec<String> = Vec::new();
    let re = Regex::new(r"((do|don't)\(\))|mul\(\d{1,3},\d{1,3}\)").unwrap();

    for mat in re.find_iter(data.as_str()) {
        let str: String = data[mat.start()..mat.end()].to_string();
        entries.push(str);
    }

    entries
}

fn convert_entry_to_num(found_matches: Vec<String>) -> i32 {
    let re = Regex::new(r"\d{1,3}").expect("Regex creation failed");

    let mut result_num = 0;
    for x in found_matches.iter() {
        let mut mul_num = 1;
        for mat in re.find_iter(x.as_str()) {
            let num = &x[mat.start()..mat.end()];
            let num = num.parse::<i32>().expect("Parsing failed");

            mul_num *= num;
        }

        if mul_num == 1 {
            continue;
        }

        result_num += mul_num;
    }

    result_num
}

fn convert_entry_to_num_with_instructions(found_matches: Vec<String>) -> i32 {
    let mut valid_instructions: Vec<String> = Vec::new();
    let mut allow_instructions: bool = true;
    for x in found_matches.iter() {
        match x.as_str() {
            "do()" => {
                allow_instructions = true;
                continue;
            }
            "don't()" => {
                allow_instructions = false;
                continue;
            }
            _ => {}
        }

        match allow_instructions {
            true => {
                valid_instructions.push(x.to_string());
            }
            false => {
                continue;
            }
        }
    }

    convert_entry_to_num(valid_instructions)
}

pub fn solve_riddle_1() {
    let solution = convert_entry_to_num(parse(get_input()));

    println!("Solution to riddle one: {}", solution)
}

pub fn solve_riddle_2() {
    let entries = parse_with_instructions(get_input());

    let solution = convert_entry_to_num_with_instructions(entries);

    println!("Solution to riddle two: {}", solution);
}

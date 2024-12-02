use std::fs::read_to_string;

fn get_input() -> String {
    match read_to_string("inputs/day_2.txt") {
        Ok(input) => input,
        Err(err) => loop {
            eprintln!("{}", err);
            std::thread::sleep(std::time::Duration::from_secs(5));
        },
    }
}

fn parse(data: String) -> Vec<Vec<u8>> {
    let mut data_set: Vec<Vec<u8>> = Vec::new();
    let mut entry: Vec<u8> = Vec::new();
    let mut str: String = String::new();
    for x in data.chars() {
        match x {
            ' ' => {
                let num = str.parse::<u8>().expect("Parsing failed");
                entry.push(num);
                str.clear();
            }
            '\n' => {
                if str.is_empty() {
                    continue;
                }

                let num = str.parse::<u8>().expect("Parsing failed");
                entry.push(num);
                data_set.push(entry.clone());
                entry.clear();
                str.clear();
            }
            _ => {
                str.push(x);
            }
        }
    }

    data_set
}

fn solve_riddle_1(data: Vec<Vec<u8>>) -> usize {
    // For second part of the exercise just count how many falses are in an entry
    // If there are more than two, then the entry stays false, otherwise it is true
    let amount = data
        .iter()
        .filter(|x| {
            let mut tmp: usize = 0;
            let mut state: bool = true;
            loop {
                if tmp + 1 == x.len() {
                    break;
                }

                if x[tmp].abs_diff(x[tmp + 1]) > 3 {
                    state = false;
                    break;
                }

                if x[tmp] > x[tmp + 1] {
                    state = true;
                } else {
                    state = false;
                    break;
                }
                tmp += 1;
            }

            if state {
                return state;
            }

            tmp = 0;
            loop {
                if tmp + 1 == x.len() {
                    break;
                }

                if x[tmp].abs_diff(x[tmp + 1]) > 3 {
                    state = false;
                    break;
                }
                if x[tmp] < x[tmp + 1] {
                    state = true;
                } else {
                    state = false;
                    break;
                }
                tmp += 1;
            }

            state
        })
        .count();
    amount
}

pub fn get_ansear_riddle_1() -> usize {
    solve_riddle_1(parse(get_input()))
}

fn solve_riddle_2(data: Vec<Vec<u8>>) -> usize {
    // For second part of the exercise just count how many falses are in an entry
    // If there are more than two, then the entry stays false, otherwise it is true

    let amount = data
        .iter()
        .filter(|x| {
            let mut tmp: usize = 0;
            let mut state: bool = true;
            let mut hits: u8 = 0;
            loop {

                if hits == 2 {
                    state = false;
                    break;
                }

                if tmp + 1 == x.len() {
                    break;
                }

                if x[tmp].abs_diff(x[tmp + 1]) > 3 {
                    hits += 1;
                }

                if x[tmp] > x[tmp + 1] {
                    state = true;
                } else {
                    state = false;
                    hits += 1;
                }
                tmp += 1;
            }

            if state {
                return state;
            }

            tmp = 0;
            hits = 0;
            loop {
                if hits == 2 {
                    state = false;
                    break;
                }

                if tmp + 1 == x.len() {
                    break;
                }

                if x[tmp].abs_diff(x[tmp + 1]) > 3 {
                    hits += 1;
                }
                if x[tmp] < x[tmp + 1] {
                    state = true;
                } else {
                    state = false;
                    hits += 1;
                }
                tmp += 1;
            }

            state
        })
        .count();
    amount
}

pub fn get_answear_riddle_2() -> usize {
    solve_riddle_2(parse(get_input()))
}

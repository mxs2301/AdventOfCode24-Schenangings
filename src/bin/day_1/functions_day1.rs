use std::fs::read_to_string;

fn get_input() -> String {
    match read_to_string("inputs/day_1.txt") {
        Ok(input) => input,
        Err(err) => loop {
            eprintln!("{}", err);
            std::thread::sleep(std::time::Duration::from_secs(5));
        },
    }
}

fn parse(data: String) -> (Vec<u32>, Vec<u32>) {
    let mut str: String = String::new();
    let mut list_a: Vec<u32> = Vec::new();
    let mut list_b: Vec<u32> = Vec::new();
    for x in data.chars() {
        match x {
            ' ' => {
                if str.is_empty() {
                    continue;
                }
                let num = str.parse::<u32>().expect("Parsing error");
                list_a.push(num);
                str.clear();
            }

            '\n' => {
                let num = str.parse::<u32>().expect("Parsing error");
                list_b.push(num);
                str.clear();
            }
            _ => {
                str.push(x);
            }
        }
    }

    (list_a, list_b)
}

fn solve_riddle(mut list_a: Vec<u32>, mut list_b: Vec<u32>) -> u32 {
    list_a.sort();
    list_b.sort();
    let iterator_len = {
        if list_a.len() >= list_b.len() {
            list_a.len()
        } else {
            list_b.len()
        }
    };

    let mut added_difference: u32 = 0;
    for x in 0..iterator_len {
        if list_a[x] >= list_b[x] {
            added_difference += list_a[x] - list_b[x];
        } else {
            added_difference += list_b[x] - list_a[x];
        }
    }

    added_difference
}

fn solve_riddle_2(mut list_a: Vec<u32>, mut list_b: Vec<u32>) -> u32 {
    list_a.sort();
    list_b.sort();

    let iterator_len = {
        if list_a.len() >= list_b.len() {
            list_a.len()
        } else {
            list_b.len()
        }
    };

    let mut result: u32 = 0;
    for index in 0..iterator_len {
        let counts = list_b.iter().filter(|f| **f == list_a[index]).count() as u32;
        result += counts * list_a[index];
    }

    result
}

pub fn get_answear_riddle_1() -> u32 {
    let (list_a, list_b) = parse(get_input());
    solve_riddle(list_a, list_b)
}

pub fn get_answear_riddle_2() -> u32 {
    let (list_a, list_b) = parse(get_input());
    solve_riddle_2(list_a, list_b)
}

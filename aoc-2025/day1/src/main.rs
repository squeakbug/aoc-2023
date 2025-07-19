use std::{
    env,
    str::Chars
};

fn parse_number(chars: &mut Chars) -> i32 {
    let (ok, _) = chars
        .into_iter()
        .try_fold((0u32, true), |(acc, _), c| {
            if let Some(d) = c.to_digit(10) {
                Ok((acc * 10 + d, true))
            } else {
                Err((acc, false))
            }
        })
        .unwrap_or_else(|acc| acc);
    ok as i32
}

fn part1(input: &str) -> String {
    let mut stream = input.chars();
    let mut zero_cnt = 0;
    let mut dial = 50;
    loop {
        let dir = match stream.next() {
            Some('L') => -1,
            Some('R') => 1,
            None => break,
            _ => panic!("Bad input")
        };
        let num = parse_number(&mut stream);
        if dir < 0 { dial -= num; } else { dial += num; }
        if dial % 100 == 0 { zero_cnt += 1; }
    }
    zero_cnt.to_string()
}

fn part2(input: &str) -> String {
    let mut stream = input.chars();
    let mut zero_cnt = 0;
    let mut dial = 50;
    loop {
        let dir = match stream.next() {
            Some('L') => -1,
            Some('R') => 1,
            None => break,
            _ => panic!("Bad input")
        };
        let mut num = parse_number(&mut stream);
        zero_cnt += num / 100;
        num = num % 100;
        if dir < 0 {
            if dial <= num && dial > 0 {
                zero_cnt += 1;
                dial += 100;
            }
            dial += 100 - num;
        } else {
            if dial + num >= 100 {
                zero_cnt += 1;
            }
            dial += num
        }
        dial %= 100;
    }
    zero_cnt.to_string()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <input_filename>", args[0]);
        return;
    }

    let filename = &args[1];
    let contents = std::fs::read_to_string(filename).expect("Error reading file");

    let part1_solution = part1(&contents);
    let part2_solution = part2(&contents);

    println!("Part 1 solution: {}", part1_solution);
    println!("Part 2 solution: {}", part2_solution);
}
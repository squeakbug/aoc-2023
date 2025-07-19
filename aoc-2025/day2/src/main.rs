use std::{
    env,
    str::Chars
};

struct Lexer<'a> {
    chars: Chars<'a>,
    curr: Option<char>,
    is_end: bool,
}

impl<'a> From<Chars<'a>> for Lexer<'a> {
    fn from(chars: Chars<'a>) -> Self {
        Lexer {
            chars,
            curr: None,
            is_end: false,
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_end { return None; }

        let mut value = match self.curr {
            Some(dig) => dig.to_digit(10).unwrap() as u64,
            None => 0u64,
        };
        while let Some(ch) = self.chars.next() {
            self.curr = Some(ch);
            if ch.is_digit(10) {
                value = value * 10 + ch.to_digit(10).unwrap() as u64;
            } else {
                self.curr = self.chars.next();
                return Some(value);
            }
        }
        self.is_end = true;
        Some(value)
    }
}

fn is_resolved(v: u64) -> bool {
    let vlen = (v.ilog10() + 1) / 2;
    let v1 = v % 10u64.pow(vlen);
    let v2 = v / 10u64.pow(vlen);
    v1 == v2
}

fn part1(input: &str) -> String {
    let mut lexer = Lexer::from(input.chars());
    let mut sum = 0u64;
    while let Some(v1) = lexer.next() {
        let v2 = lexer.next().unwrap();
        for v in v1..=v2 {
            if is_resolved(v) {
                sum += v;
            }
        }
    }
    sum.to_string()
}

fn is_resolved2(v: u64) -> bool {
    let vlen = v.ilog10() + 1;
    for i in 2..=vlen {
        if vlen % i != 0 { continue; }
        let vleni = 10u64.pow(vlen / i);
        let mut is_find = true;

        let mut vtmp = v;
        let mut v1 = vtmp % vleni;
        vtmp /= vleni;
        for _ in 0..i-1 {
            let v2 = v1;
            v1 = vtmp % vleni;
            vtmp /= vleni;

            if v1 != v2 || (v1 == 0 && v2 == 0) { 
                is_find = false;
                break;
            }
        }
        if is_find {
            return true;
        }
    }
    return false;
}

fn part2(input: &str) -> String {
    let mut lexer = Lexer::from(input.chars());
    let mut sum = 0u64;
    while let Some(v1) = lexer.next() {
        let v2 = lexer.next().unwrap();
        for v in v1..=v2 {
            if is_resolved2(v) {
                sum += v;
            }
        }
    }
    sum.to_string()
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
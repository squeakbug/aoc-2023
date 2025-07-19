use std::env;

fn find_max(input: &str, begin: usize, end: usize) -> (u32, usize) {
    let mut fst_dig = 0;
    let mut fst_indx = 0;
    let mut indx = 0;
    let mut stream = input.chars();
    while let Some(ch) = stream.next() {
        if indx >= begin && indx < end {
            let dig = ch.to_digit(10).unwrap();
            if dig > fst_dig {
                fst_dig = dig;
                fst_indx = indx;
            }
        }
        indx += 1;
    }
    (fst_dig, fst_indx)
}

fn get_value(input: &str) -> u32 {
    let (fst_dig, fst_indx) = find_max(input, 0, input.len());
    if fst_indx == input.len() - 1 {
        let (snd_dig, _) = find_max(input, 0, fst_indx);
        snd_dig * 10 + fst_dig
    } else {
        let (snd_dig, _) = find_max(input, fst_indx + 1, input.len());
        fst_dig * 10 + snd_dig
    }
}

fn part1(input: &str) -> String {
    let lines = input.split("\n");
    let mut sum = 0;
    for line in lines {
        let v = get_value(line);
        sum += v;
    }
    sum.to_string()
}

fn part2(_input: &str) -> String {
    String::from("Hello world!")
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
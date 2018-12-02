use std::fs;
use counter::Counter;
use std::collections::HashSet;
use itertools::Itertools;

fn read_input() -> Vec<String> {
    let contents = fs::read_to_string("input.txt").expect("File read error");
    contents.lines().map(|x| x.to_string()).collect()
}

fn main() {
    let lines = read_input();

    let mut two_count = 0;
    let mut three_count = 0;

    for line in lines.iter() {
        let char_counts = line.chars().collect::<Counter<_>>();
        let counts_set = char_counts.values().collect::<HashSet<_>>();
        if counts_set.contains(&2) {
            two_count += 1;
        }
        if counts_set.contains(&3) {
            three_count += 1;
        }
    }
    let result = two_count * three_count;
    println!("{}", result);

    for (a, b) in lines.iter().tuple_combinations() {
        let same: String = a.chars().zip(b.chars())
            .filter(|(c1, c2)| c1 == c2)
            .map(|(c1, _)| c1)
            .collect();
        if a.len() - same.len() == 1 {
            println!("{}", same);
            break
        }
    }
    
}

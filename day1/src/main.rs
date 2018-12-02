use std::fs;
use std::collections::HashSet;

fn read_input() -> Vec<String> {
    let contents = fs::read_to_string("input.txt").expect("File read error");
    contents.lines().map(|x| x.to_string()).collect()
}

fn main() {
    let lines = read_input();

    let mut result = 0;
    for line in lines.iter() {
        let value: i32 = line.parse().unwrap();
        result += value;
    }
    println!("{}", result);

    let mut result = 0;
    let mut seen: HashSet<i32> = HashSet::new();
    seen.insert(result);
    loop {
        for line in lines.iter() {
            let value: i32 = line.parse().unwrap();
            result += value;
            if seen.contains(&result) {
                println!("{}", result);
                return;
            }
            seen.insert(result);
        }
    }
    
}

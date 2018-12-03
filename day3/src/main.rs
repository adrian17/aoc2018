use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use regex::Regex;

fn read_input() -> Vec<String> {
    let contents = fs::read_to_string("input.txt").expect("File read error");
    contents.lines().map(|x| x.to_string()).collect()
}

fn main() {
    let lines = read_input();

    let pattern = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();

    let mut map: HashMap<(i32, i32), Vec<i32>> = HashMap::new();
    let mut no_collisions: HashSet<i32> = HashSet::new();

    for line in lines.iter() {
        let captures = pattern.captures(line).unwrap();
        let claim: i32 = captures[1].parse().unwrap();
        let x: i32 = captures[2].parse().unwrap();
        let y: i32 = captures[3].parse().unwrap();
        let w: i32 = captures[4].parse().unwrap();
        let h: i32 = captures[5].parse().unwrap();

        no_collisions.insert(claim);

        for nx in x..x+w {
            for ny in y..y+h {
                let claims = map.entry((nx, ny)).or_default();
                claims.push(claim);

                if claims.len() > 1 {
                    for claim in claims {
                        no_collisions.remove(claim);
                    }
                }
            }
        }
    }
    let result = map.values().filter(|x| x.len() > 1).count();
    println!("{}", result);

    println!("{:?}", no_collisions);
}

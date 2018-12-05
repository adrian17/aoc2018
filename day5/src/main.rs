use std::fs;

const ALPHA: &str = "abcdefghijklmnopqrstuvwxyz";

fn read_input() -> Vec<String> {
    let contents = fs::read_to_string("input.txt").expect("File read error");
    contents.lines().map(|x| x.to_string()).collect()
}

fn opposite(c1: char, c2: char) -> bool {
    c1 != c2 && c1.eq_ignore_ascii_case(&c2)
}

fn shorten(chain: &str) -> String {
    chain.chars().fold("".to_string(), |mut sum, c| {
        if !sum.is_empty() && opposite(c, sum.chars().last().unwrap()) {
            sum.pop();
        } else {
            sum.push(c);
        }
        sum
    })
}

fn main() {
    let lines = read_input();
    assert!(lines.len() == 1);

    let chain = &lines[0];
    let result = shorten(chain);

    println!("{}", result.len());

    let mut best = chain.len();
    for c in ALPHA.chars() {
        let new = chain.replace(c, "").replace(&c.to_uppercase().to_string(), "");
        let shortened = shorten(&new);
        if shortened.len() < best {
            best = shortened.len();
        }
    }
    println!("{}", best);
}

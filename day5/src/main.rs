use std::fs;

const ALPHA: &str = "abcdefghijklmnopqrstuvwxyz";

fn read_input() -> Vec<String> {
    let contents = fs::read_to_string("input.txt").expect("File read error");
    contents.lines().map(|x| x.to_string()).collect()
}

fn iterate(chain: &str) -> String {
    let mut ret = String::new();

    for c in chain.chars() {
        if ret.is_empty() {
            ret.push(c);
        } else {
            let last = ret.chars().last().unwrap();
            if c.eq_ignore_ascii_case(&last) && c.is_lowercase() != last.is_lowercase() {
                ret.pop();
            } else {
                ret.push(c);
            }
        }
    }
    ret
}

fn shorten(chain: String) -> String {
    let mut previous = chain;
    let mut next;
    loop {
        next = iterate(&previous);
        if next.len() == previous.len() {
            return previous;
        }
        previous = next;
    }
}

fn main() {
    let lines = read_input();
    assert!(lines.len() == 1);

    let chain = lines[0].to_string();
    let result = shorten(chain.clone());

    println!("{}", result.len());

    let mut best = chain.len();
    for c in ALPHA.chars() {
        let new = chain.replace(c, "").replace(&c.to_uppercase().to_string(), "");
        let shortened = shorten(new);
        if shortened.len() < best {
            best = shortened.len();
        }
    }
    println!("{}", best);
}

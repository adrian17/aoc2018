use std::fs;
use std::collections::HashMap;
use regex::Regex;

fn read_input() -> Vec<String> {
    let contents = fs::read_to_string("input.txt").expect("File read error");
    contents.lines().map(|x| x.to_string()).collect()
}

fn main() {
    let mut lines = read_input();
    lines.sort();

    let pattern = Regex::new(r"\[.+:(\d+)\] (.+)").unwrap();

    let mut person = -1;
    let mut start_minute = -1;
    let mut people: HashMap<i32, Vec<_>> = HashMap::new();

    for line in lines {
        let captures = pattern.captures(&line).unwrap();
        let minute: i32 = captures[1].parse().unwrap();
        let text = captures[2].to_string();

        match text.as_ref() {
            "falls asleep" => start_minute = minute,
            "wakes up" => people.entry(person).or_default().push((start_minute, minute)),
            _ => {
                let parts: Vec<_> = text.split_whitespace().collect();
                person = parts[1].trim_start_matches('#').parse().unwrap();
            }
        }
    }

    let (best_person_id, best_person_periods) = people.iter().max_by_key(|(_, periods)| -> i32 {
        periods.iter().map(|(start, end)| end-start).sum()
    }).unwrap();

    let mut counts: HashMap<i32, i32> = HashMap::new();
    for &(start, end) in best_person_periods {
        for minute in start..end {
            *counts.entry(minute).or_default() += 1;
        }
    }
    let (best_minute, _) = counts.iter().max_by_key(|&(_, value)| value).unwrap();
    println!("{}", best_person_id * best_minute);

    let mut counts: HashMap<(i32, i32), i32> = HashMap::new();
    for (person, periods) in people {
        for (start, end) in periods {
            for minute in start..end {
                *counts.entry((person, minute)).or_default() += 1;
            }
        }
    }
    let ((best_person, best_minute), _) = counts.iter().max_by_key(|&(_, value)| value).unwrap();
    println!("{}", best_person * best_minute);
}

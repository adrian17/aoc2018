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

    let pattern = Regex::new(r"Step (\w) must be finished before step (\w) can begin.").unwrap();

    let mut graph: HashMap<char, Vec<char>> = HashMap::new();

    for line in lines {
        let captures = pattern.captures(&line).unwrap();
        let source: char = captures[1].parse().unwrap();
        let destination: char = captures[2].parse().unwrap();

        graph.entry(source).or_default().push(destination);
        graph.entry(destination).or_default();
    }
    part1(&graph);
    part2(&graph);

}

fn part1(graph: &HashMap<char, Vec<char>>) {
    let mut graph = graph.clone();
    let mut output = String::new();

    while !graph.is_empty() {
        let sources: HashSet<char> = graph.keys().cloned().collect();
        let destinations: HashSet<char> = graph.values().flatten().cloned().collect();
        let origin = sources.difference(&destinations).min().unwrap();
        graph.remove(origin);
        output.push(*origin);
    }
    println!("{}", output);
}

fn part2(graph: &HashMap<char, Vec<char>>) {
    let mut graph = graph.clone();

    let mut t = 0;
    let mut workers = [None as Option<(char, i32)>; 5];
    let mut taken = HashSet::new();

    while !graph.is_empty() {
        for worker in &mut workers {
            if let Some((task, end_time)) = *worker {
                if t == end_time {
                    *worker = None;
                    graph.remove(&task);
                    taken.remove(&task);
                }
            }
        }
        for worker in &mut workers {
            if *worker == None {
                let sources: HashSet<char> = graph.keys().cloned().collect();
                let destinations: HashSet<char> = graph.values().flatten().cloned().collect();
                if let Some(origin) = sources.difference(&destinations).cloned().filter(|c| !taken.contains(c)).min() {
                    let dt = (origin as u32) - ('A' as u32) + 1;
                    *worker = Some((origin, t + 60 + dt as i32));
                    taken.insert(origin);
                }
            }
        }
        t += 1;
    }
    println!("{}", t - 1);

}


use std::fs;
use std::collections::HashMap;

fn read_input() -> Vec<String> {
    let contents = fs::read_to_string("input.txt").expect("File read error");
    contents.lines().map(|x| x.to_string()).collect()
}

fn main() {
    let lines = read_input();
    let mut grid: Vec<_> = lines[0][15..].chars().collect();
    let rules: HashMap<_, _> = lines[2..].iter().map(|line| {
        let parts: Vec<_> = line.split(" => ").collect();
        (parts[0].to_string(), parts[1].parse::<char>().unwrap())
    }).collect();
    println!("{:?}", grid.iter().collect::<String>());

    let mut offset = 0;

    for iter in 0..200 {
        while grid[0..5].contains(&'#') {
            grid.insert(0, '.');
            offset -= 1;
        }
        while grid[grid.len()-5..grid.len()].contains(&'#') {
            grid.push('.');
        }
        let mut new_grid = Vec::new();
        for window in grid.windows(5) {
            let pattern: String = window.iter().collect();
            if let Some(&new) = rules.get(&pattern) {
                new_grid.push(new);
            } else {
                new_grid.push(window[2]);
            }
        }
        grid = new_grid;
        offset += 2;
        
        let sum: i32 = grid.iter().cloned()
            .enumerate()
            .filter(|&(_, e)| e == '#')
            .map(|(i, _)| (i as i32) + offset)
            .sum();
        println!("{} {:?} {}", iter+1, grid.iter().collect::<String>(), sum);
    }
    // at this point I manually looked at the results
    let diff = 19614 - 19528;
    let part2 = 19614 + (50000000000usize - 200) * diff;
    println!("{}", part2);
}

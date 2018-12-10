use std::fs;
use regex::Regex;
use std::collections::HashSet;
use std::{thread, time};

fn read_input() -> Points {
    let pattern = Regex::new(r"[-0-9]+").unwrap();
    let contents = fs::read_to_string("input.txt").expect("File read error");
    contents.lines().map(|line| {
        let nums: Vec<_> = pattern.find_iter(&line).map(|x| x.as_str().parse::<i32>().unwrap()).collect();
        Point{x: nums[0], y: nums[1], vx: nums[2], vy: nums[3]}
    }).collect()
}

struct Point{x: i32, y: i32, vx: i32, vy: i32}
type Points = Vec<Point>;

fn update(points: &mut Points) {
    for point in points {
        point.x += point.vx;
        point.y += point.vy;
    }
}

fn print(count: i32, points: &[Point]) {
    let min_x = points.iter().map(|point| point.x).min().unwrap();
    let max_x = points.iter().map(|point| point.x).max().unwrap();
    let min_y = points.iter().map(|point| point.y).min().unwrap();
    let max_y = points.iter().map(|point| point.y).max().unwrap();
    let w = (max_x - min_x + 1) as usize;
    let h = (max_y - min_y + 1) as usize;
    if w < 70 && h < 70 {
        let points_set: HashSet<_> = points.iter().map(|point| (point.x, point.y)).collect();
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let c = if points_set.contains(&(x, y)) {'#'} else {'.'};
                print!("{}", c);
            }
            println!();
        }
        println!("{}", count);
        println!();
        thread::sleep(time::Duration::from_millis(1000));
    }
}

fn main() {
    let mut points = read_input();
    let mut count = 0;
    loop {
        print(count, &points);
        count += 1;
        update(&mut points);
    }
}

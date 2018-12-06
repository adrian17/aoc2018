use std::fs;
use std::collections::HashSet;
use std::ops::Sub;

fn read_input() -> Vec<String> {
    let contents = fs::read_to_string("input.txt").expect("File read error");
    contents.lines().map(|x| x.to_string()).collect()
}

fn parse(lines: &[String]) -> Vec<Point> {
    lines.iter().map(|line| {
        let parts: Vec<_> = line.split(", ").map(|x| x.parse::<i32>().unwrap()).collect();
        Point {x: parts[0], y: parts[1]}
    }).collect()
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
impl Sub for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Point {
        Point {x: self.x - other.x, y: self.y - other.y}
    }
}
impl Point {
    fn len(self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

fn main() {
    let lines = read_input();
    let points = parse(&lines);

    let min_x = points.iter().map(|point| point.x).min().unwrap();
    let max_x = points.iter().map(|point| point.x).max().unwrap();
    let min_y = points.iter().map(|point| point.y).min().unwrap();
    let max_y = points.iter().map(|point| point.y).max().unwrap();

    let mut excludes: HashSet<usize> = HashSet::new();
    let mut scores = vec![0; points.len()];

    let mut part_2_region_size = 0;

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let location = Point{x, y};
            let distances: Vec<i32> = points.iter().map(|&p| (p-location).len()).collect();
            let best_distance = distances.iter().min().unwrap();
            let best_points: Vec<_> = distances.iter().enumerate()
                .filter(|&(_, e)| e == best_distance)
                .map(|(i, _)| i)
                .collect();
            if best_points.len() == 1 {
                let best_idx = best_points[0];
                scores[best_idx] += 1;
                if x == min_x || x == max_x || y == min_y || y == max_y {
                    excludes.insert(best_idx);
                }
            }
            let distances_sum: i32 = distances.iter().sum();
            if distances_sum < 10000 {
                part_2_region_size += 1;
            }
        }
    }
    let best_value = scores.iter().enumerate()
        .filter(|&(i, _)| !excludes.contains(&i))
        .map(|(_, e)| e)
        .max().unwrap();
    println!("{:?}", best_value);

    println!("{:?}", part_2_region_size);
}

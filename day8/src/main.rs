use std::fs;

fn read_input() -> Vec<String> {
    let contents = fs::read_to_string("input.txt").expect("File read error");
    contents.lines().map(|x| x.to_string()).collect()
}

struct Node {
    children: Vec<Node>,
    metadata: Vec<i32>
}

fn parse(nums: &[i32], mut i: usize) -> (Node, usize) {
    let mut node = Node {children: Vec::new(), metadata: Vec::new()};
    let (nchild, nmeta) = (nums[i], nums[i+1]);
    i += 2;
    for _ in 0..nchild {
        let (new_node, new_i) = parse(nums, i);
        i = new_i;
        node.children.push(new_node);
    }
    for _ in 0..nmeta {
        let value = nums[i];
        i += 1;
        node.metadata.push(value);
    }
    (node, i)
}

fn part1(node: &Node) -> i32 {
    let sum_children: i32 = node.children.iter().map(|x| part1(x)).sum();
    let sum_metadata: i32 = node.metadata.iter().sum();
    sum_children + sum_metadata
}

fn part2(node: &Node) -> i32 {
    if node.children.is_empty() {
        return node.metadata.iter().sum();
    }
    node.metadata.iter().map(|&x| {
        if (x as usize) - 1 < node.children.len() {
            part2(&node.children[x as usize - 1])
        } else { 0 }
    }).sum()
}

fn main() {
    let line = &read_input()[0];
    let nums: Vec<_> =
        line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
    let (points, _) = parse(&nums, 0);

    println!("{}", part1(&points));
    println!("{}", part2(&points));
}

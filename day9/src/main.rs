use std::collections::VecDeque;

const N_PLAYERS: usize = 411;
const LAST: usize = 7117000;

fn rotate(marbles: &mut VecDeque<usize>, n: i32) {
    if n > 0 {
        for _ in 0..n {
            let value = marbles.pop_front().unwrap();
            marbles.push_back(value);
        }
    } else {
        for _ in 0..(-n) {
            let value = marbles.pop_back().unwrap();
            marbles.push_front(value);
        }
    }
}

fn main() {
    let mut marbles = VecDeque::new();
    marbles.push_back(0usize);

    let mut players = vec![0usize; N_PLAYERS];
    let mut player: usize = 0;

    for marble in 1..=LAST {
        if marble % 23 == 0 {
            rotate(&mut marbles, -7);
            players[player] += marble;
            players[player] += marbles.pop_front().unwrap();
        } else {
            rotate(&mut marbles, 2);
            marbles.push_front(marble);
        }
        player = (player + 1) % players.len();
    }
    let max_score = players.iter().max().unwrap();
    println!("{}", max_score);
}

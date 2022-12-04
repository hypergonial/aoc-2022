use std::fs;

fn to_score(text: &str) -> u32 {
    if text == "A" || text == "X" {
        1
    } else if text == "B" || text == "Y" {
        2
    } else {
        3
    }
}

/// Get the score for a single game.
fn get_score(a: u32, b: u32) -> u32 {
    match (a, b) {
        (1, 3) => b,
        (2, 1) => b,
        (3, 2) => b,
        (x, y) if x == y => b + 3,
        _ => b + 6,
    }
}

/// Get the desired move based on the required outcome (b).
fn get_move(a: u32, b: u32) -> Option<u32> {
    match b {
        1 => if a - 1 == 0 {Some(3)} else {Some(a - 1)}
        2 => Some(a),
        3 => if a + 1 > 3 {Some(1)} else {Some(a + 1)}
        _ => None,
    }
}

pub fn run() {
    let lines = fs::read_to_string("input/day2in.txt").expect("Failed to read file!");
    let mut score = 0;
    let mut score_alt = 0;

    for line in lines.split('\n') {
        let moves = line.split(' ').take(2).collect::<Vec<&str>>();
        let a = to_score(moves[0]);
        let b = to_score(moves[1]);

        score += get_score(a, b);
        score_alt += get_score(a, get_move(a, b).unwrap());
    }
    println!("{score} {score_alt}");
}

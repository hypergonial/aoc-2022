use std::fs;

const ABC: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn push_score(text: &[&str], scores: &mut Vec<u32>) {
    for char in ABC.chars() {
        if text.iter().all(|x| x.contains(char)) {
            let score = ABC.find(char).expect("Invalid character found!") as u32 + 1;
            scores.push(score);
        }
    }
}

pub fn run() {
    let file = fs::read_to_string("input/day3in.txt").expect("Failed to read file!");
    let lines = file.split("\n").collect::<Vec<&str>>();

    let mut scores: Vec<u32> = Vec::new();
    let mut badge_scores: Vec<u32> = Vec::new();

    for line in &lines {
        let (first, second) = line.split_at(line.len() / 2);
        push_score(&[first, second], &mut scores);
    }
    for chunk in lines.chunks(3) {
        push_score(chunk, &mut badge_scores);
    }
    println!(
        "{} {}",
        scores.iter().sum::<u32>(),
        badge_scores.iter().sum::<u32>()
    );
}

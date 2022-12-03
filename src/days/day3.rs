use std::collections::HashMap;
use std::fs;

use lazy_static::lazy_static;

lazy_static!{
    static ref SCORE_MAPPING: HashMap<char, u32> = {
        let mut map = HashMap::new();
        for (char, score) in ('a'..='z').zip(1..).chain(('A'..='Z').zip(27..)) {
            map.insert(char, score);
        }
        map
    };
}

fn push_common_occurences(text: &[&str], scores: &mut Vec<u32>) {
    for char in SCORE_MAPPING.keys() {
        if text.iter().all(|x| x.contains(*char)) {
            let score = SCORE_MAPPING.get(char).expect("Invalid character found!").to_owned();
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
        let (first, second) = line.split_at(line.len()/2);
        push_common_occurences(&[first, second], &mut scores);
    }
    for chunk in lines.chunks(3) {
        push_common_occurences(chunk, &mut badge_scores);
    }
    println!("{} {}", scores.iter().sum::<u32>(), badge_scores.iter().sum::<u32>());   
}

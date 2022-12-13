use std::fs;
use take_until::TakeUntilExt;

fn is_visible(height: u8, (x, y): (usize, usize), trees: &Vec<Vec<u8>>) -> bool {
    if x == 0 || x == trees[x].len()-1 || y == 0 || y == trees.len()-1 {
        return true;
    }
    let x1_max = trees[y][0..x].iter().max().unwrap();
    let x2_max = trees[y][(x+1)..trees[y].len()].iter().max().unwrap();
    let y1_max = trees[0..y].iter().map(|h| h[x]).max().unwrap();
    let y2_max = trees[(y+1)..trees[y].len()].iter().map(|h| h[x]).max().unwrap();

    height > *x1_max || height > *x2_max || height > y1_max || height > y2_max
}

fn get_scenic_score(height: u8, (x, y): (usize, usize), trees: &[Vec<u8>]) -> usize {
    let x1 = trees[y][0..x].iter().rev().take_until(|h| **h >= height).count();
    let x2 = trees[y][(x+1)..trees[y].len()].iter().take_until(|h| **h >= height).count();
    let y1 = trees[0..y].iter().map(|h| h[x]).rev().take_until(|h| *h >= height).count();
    let y2 = trees[(y+1)..trees[y].len()].iter().map(|h| h[x]).take_until(|h| *h >= height).count();
    x1*x2*y1*y2
}


pub fn run() {
    let lines = fs::read_to_string("input/day8in.txt").expect("Failed to read file!");
    let mut trees: Vec<Vec<u8>> = Vec::new();

    for line in lines.split('\n') {
        trees.push(line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect::<Vec<u8>>());
    }

    let mut sum = 0;
    let mut scores: Vec<usize> = Vec::new();

    for (i, line) in trees.iter().enumerate() {
        for (j, tree) in line.iter().enumerate() {
            if is_visible(*tree, (j, i), &trees) {
                sum += 1;
            }
            scores.push(get_scenic_score(*tree, (j, i), &trees))
        }
    }
    println!("{} {}", sum, scores.iter().max().unwrap());
}
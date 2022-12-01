use std::fs;

pub fn run() {
    let lines = fs::read_to_string("input/day1in.txt").expect("Where file");
    let mut elves: Vec<i32> = vec![0];

    for line in lines.split("\n") {
        if line != "" {
            let len = elves.len();
            elves[len-1] += line.parse::<i32>().expect("You better be an integer");
        }
        else {
            elves.push(0);
        }
    }
    elves.sort();
    let maxes = elves.into_iter().rev().take(3).collect::<Vec<i32>>();

    // Task 1
    println!("{}", maxes[0]);
    // Task 2
    println!("{}", maxes.iter().sum::<i32>());
    
}
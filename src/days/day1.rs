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
    let max1 = elves.iter().max().expect("Expected at least one value.");
    let max2 = elves.iter().filter(|x| *x != max1).max().expect("Expected at least two values.");
    let max3 = elves.iter().filter(|x| *x != max1 && *x != max2).max().expect("Expected at least three values.");

    // Task 1
    println!("{}", max1);
    // Task 2
    println!("{}", max1+max2+max3);
    
}
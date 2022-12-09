use std::{fs, collections::HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
struct Coordinate {x: i32, y: i32}

fn do_follow(head: &Coordinate, tail: &mut Coordinate) {
    if (head.x-tail.x).abs().max((head.y-tail.y).abs()) <= 1 {
        return
    }
    tail.x += (head.x-tail.x).signum();
    tail.y += (head.y-tail.y).signum();
}

fn execute_instruction(rope: &mut Vec<Coordinate>, instruction: &str, visited: &mut HashSet<Coordinate>){
    let instruction: Vec<&str> = instruction.split(' ').collect();
    let (dir, chg) = (instruction[0], instruction[1].parse::<i32>().unwrap());

    let change_coord: Box<dyn Fn(&mut Coordinate)> = match dir {
        "R" => Box::new(|obj| obj.x += 1),
        "U" => Box::new(|obj| obj.y += 1),
        "D" => Box::new(|obj| obj.y -= 1),
        "L" => Box::new(|obj| obj.x -= 1),
        _ => panic!("Invalid direction specified!"),
    };

    for _ in 0..chg {
        change_coord(&mut rope[0]);

        for i in 0..(rope.len()-1) {
            let needs_follow = rope[i];
            do_follow(&needs_follow, &mut rope[i+1]);
        }
        visited.insert(rope[rope.len()-1]);
    }
}

pub fn run() {
    let lines = fs::read_to_string("input/day9in.txt").expect("Failed to read file!");
    let mut rope: Vec<Coordinate> = vec![Coordinate::default(); 10];
    let mut visited: HashSet<Coordinate> = HashSet::from([Coordinate::default()]);

    for line in lines.split('\n') {
        execute_instruction(&mut rope, line, &mut visited);
    }
    println!("Counter: {}", visited.len());
}

use std::{fs, collections::VecDeque};

fn do_move((amount, from, to): &(usize, usize, usize), cols: &mut Vec<VecDeque<char>>) {
    for _ in 0..*amount {
        let item = cols[from-1].pop_front().unwrap();
        cols[to-1].push_front(item);
    }
    
}

fn do_multi_move((amount, from, to): &(usize, usize, usize), cols: &mut Vec<VecDeque<char>>) {
    let mut buf: Vec<char> = Vec::new();
    for _ in 0..*amount {
        buf.push(cols[from-1].pop_front().unwrap())
    }
    for item in buf.iter().rev() {
        cols[to-1].push_front(*item);
    }
}

fn get_top_boxes(cols: &Vec<VecDeque<char>>) -> String {
    cols.iter().map(|col| col.front().expect("Empty column detected!")).collect()
}

pub fn run() {
    let lines = fs::read_to_string("input/day5in.txt").expect("Failed to read file!");

    // each inner Vec is a line, spaces represent no box at that pos, last line is popped for count
    let mut front: Vec<Vec<char>> = lines.split('\n').take_while(|x| !x.is_empty()).map(|line| {
        line.chars().enumerate().filter(|(i, _)| *i > 0 && (i-1)%4 == 0).map(|(_, s)| s).collect::<Vec<char>>()
    }
    ).collect();
    let count = front.pop().unwrap().len();

    // (amount, from, to)
    let moves: Vec<(usize, usize, usize)> = lines.split('\n').skip_while(|x| !x.is_empty()).skip(1).map(|line| {
        let parsed = line.replace("move ", "").replace(" from", "").replace(" to", "").split(' ').map(|n| n.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        (parsed[0], parsed[1], parsed[2])
    }).collect();

    // Each VecDeque is one column of boxes
    let mut cols = Vec::new();
    for i in 0..count {
        cols.push(VecDeque::new());
        for row in &front {
            if row[i] != ' ' {
                cols.last_mut().unwrap().push_back(row[i]);
            }
        }
    }
    let mut multi_cols = cols.clone();

    for mv in &moves {
        do_move(mv, &mut cols);
    }
    for mv in &moves {
        do_multi_move(mv, &mut multi_cols);
    }
    println!("{:?} {:?}", get_top_boxes(&cols), get_top_boxes(&multi_cols));
}
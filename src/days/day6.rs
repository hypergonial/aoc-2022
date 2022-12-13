use std::fs;
use std::collections::HashSet;
use std::hash::Hash;

fn find_unique_slice<T: Hash + Eq>(arr: &[T], len: usize) -> Option<usize> {
    let mut index = None;
    'outer: for i in (len-1)..arr.len() {
        let mut buf: HashSet<&T> = HashSet::new();
        
        for item in arr.iter().take(i + 1).skip(i-(len-1)) {
            if !buf.insert(item) {
                continue 'outer;
            }
        }
        index = Some(i+1);
        break;
    }
    index
}

pub fn run() {
    let line: Vec<char> = fs::read_to_string("input/day6in.txt").expect("Failed to read file!").chars().collect();
    let index = find_unique_slice(&line, 4);
    let long_index = find_unique_slice(&line, 14);
    println!("{} {}", index.expect("No unique combo found."), long_index.expect("No long unique combo found."));
}
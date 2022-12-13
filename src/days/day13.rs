use std::{fs, cmp::Ordering};
use serde_json::{Value};

/// Compare two slices of Value.
fn compare(left: &[Value], right: &[Value]) -> Ordering {
    for i in 0.. {
        let l = left.get(i);
        let r = right.get(i);

        // Both ran out
        if l.is_none() && r.is_none() {
            return Ordering::Equal;
        }
        // First list ran out
        if l.is_none() && r.is_some() {
            return Ordering::Less;
        }
        // Second list ran out
        if r.is_none() {
            return Ordering::Greater;
        }

        let ordering = match (l.unwrap(), r.unwrap()) {
            (Value::Number(x), Value::Number(y)) => {
                x.as_i64().unwrap().cmp(&y.as_i64().unwrap())
            }
            (Value::Array(x), Value::Array(y)) => {
                compare(x, y)
            }
            (Value::Number(_), Value::Array(y)) => {
                compare(&[l.unwrap().clone()], y)
            }
            (Value::Array(x), Value::Number(_)) => {
                compare(x, &[r.unwrap().clone()])
            }
            _ => panic!("Invalid component in array!")
        };
        let Ordering::Equal = ordering else {
            return ordering;
        };
    }
    Ordering::Equal
}

pub fn run() {
    let mut lines = fs::read_to_string("input/day13in.txt").expect("Failed to read file!");
    let mut sorted_pairs = Vec::new();

    for (i, chunk) in lines.split('\n').filter(|l| !l.trim().is_empty()).collect::<Vec<&str>>().chunks(2).enumerate() {
        let (left, right): (Value, Value) = (serde_json::from_str(chunk[0]).unwrap(), serde_json::from_str(chunk[1]).unwrap());
        let Value::Array(left_vec) = left else {panic!("Among Us")};
        let Value::Array(right_vec) = right else {panic!("Among Us")};

        if let Ordering::Less = compare(&left_vec, &right_vec) {
            sorted_pairs.push(i+1);
        }
    }
    // Task 1
    println!("{}", sorted_pairs.iter().sum::<usize>());

    // Task 2
    lines.push_str("\n[[2]]\n[[6]]");
    let mut values = lines.split('\n').filter(|l| !l.trim().is_empty()).map(|l| {
        let Value::Array(arr) = serde_json::from_str(l).unwrap() else {panic!("Among Us")};
        arr
    }).collect::<Vec<_>>();

    values.sort_by(|l, r| compare(l, r));

    let packet1: Vec<Value> = vec![serde_json::from_str("[2]").unwrap()];
    let packet2: Vec<Value> = vec![serde_json::from_str("[6]").unwrap()];

    let key = (values.iter().position(|v| v == &packet1).unwrap()+1) * (values.iter().position(|v| v == &packet2).unwrap()+1);
    
    println!("{}", key);

    
}
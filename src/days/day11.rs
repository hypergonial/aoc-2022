use std::{fs, collections::VecDeque};
use std::sync::Arc;


#[derive(Clone)]
struct Monke {
    items: VecDeque<isize>,
    // Arc so I can clone it, totally not hackyTM
    operation: Arc<Box<dyn (Fn(isize) -> isize)>>,
    div_by: isize,
    true_throw: usize,
    false_throw: usize,
    counter: usize,
}

impl Monke {
    /// Execute all associated logic with the monke.
    fn throw_stuff_at(&mut self, others: &mut [Monke], magic_number: isize) {
        while let Some(item) = self.items.pop_front() {
            let new = (self.operation)(item) % magic_number;

            if new % self.div_by == 0 {
                others[self.true_throw].items.push_back(new);
            }
            else {
                others[self.false_throw].items.push_back(new);
            }
            self.counter += 1;
        }
    }
}

impl Default for Monke {
    fn default() -> Self {
        Monke {
            items: VecDeque::new(), 
            operation: Arc::new(Box::new(|x| x)), 
            div_by: 0, 
            true_throw: 0, 
            false_throw: 0, 
            counter: 0
        }
    }
}

/// Iter through all monkes and execute their logic.
fn perform_turn(monkes: &mut [Monke], magic_number: isize) {
    for i in 0..monkes.len() {
        let mut monke = monkes[i].clone();
        monke.throw_stuff_at(monkes, magic_number);
        monkes[i] = monke;
    }
}

pub fn run() {
    let mut monkes: Vec<Monke> = Vec::new();
    let file = fs::read_to_string("input/day11in.txt").expect("Failed to read file!");
    let lines = file.split('\n').collect::<Vec<&str>>();
    let monke_data = lines.split(|x| x.is_empty()).collect::<Vec<_>>();
    // Multiplying all the modulo operands together
    let mut magic_number = 1;

    for monke in monke_data {
        let mut new = Monke::default();

        for (i, data) in monke.iter().enumerate() {
            match i {
                0 => continue,
                1 => {
                    new.items = data.trim().replace("Starting items: ", "").split(", ").map(|num| num.parse::<isize>().unwrap()).collect::<VecDeque<isize>>();
                },
                2 => {
                    let trimmed = data.trim().replace("Operation: new = old ", "");
                    let op = trimmed.split(' ').collect::<Vec<&str>>();

                    if op[0] == "*" {
                        let num = if op[1] != "old" {Some(op[1].parse::<isize>().unwrap())} else {None};
                        new.operation = Arc::new(Box::new(move |x| x * num.unwrap_or(x)));
                    }
                    else {
                        let num = if op[1] != "old" {Some(op[1].parse::<isize>().unwrap())} else {None};
                        new.operation = Arc::new(Box::new(move |x| x + num.unwrap_or(x)));
                    }
                },
                3 => {
                    new.div_by = data.trim().replace("Test: divisible by ", "").parse::<isize>().unwrap();
                    magic_number *= new.div_by;
                },
                4 => {
                    new.true_throw = data.trim().chars().last().unwrap().to_digit(10).unwrap() as usize;
                },
                5 => {
                    new.false_throw = data.trim().chars().last().unwrap().to_digit(10).unwrap() as usize;
                },
                _ => panic!("Too much data!"),
            }
        }
        monkes.push(new);
    }

    for _ in 0..10000 {
        perform_turn(&mut monkes, magic_number);
    }

    let mut serious_business = monkes.iter().map(|m| m.counter).collect::<Vec<usize>>();
    serious_business.sort();

    println!("{:#?}", serious_business.iter().rev().take(2).product::<usize>());    
}
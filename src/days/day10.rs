use std::fs;

struct Memory {
    x: isize,
    cycle: usize,
    clocks: Vec<isize>,
    display: Vec<String>,
    cursor: (usize, usize),
}

impl Memory {
    fn new() -> Self {
        Memory {x: 1, cycle: 0, clocks: Vec::new(), display: vec![String::new()], cursor: (0,0)}
    }
}

fn do_cycle(mem: &mut Memory) {
    mem.cycle += 1;
    if mem.cycle%40 == 20 {
        mem.clocks.push(mem.x * mem.cycle as isize)
    }
    render(mem);
}

fn addx(mem: &mut Memory, val: isize) {
    do_cycle(mem);
    do_cycle(mem);
    mem.x += val
}

fn noop(mem: &mut Memory) {
    do_cycle(mem);
}

fn render(mem: &mut Memory) {
    if (mem.x-mem.cursor.0 as isize).abs() <= 1 {
        mem.display[mem.cursor.1].push('#');
    }
    else {
        mem.display[mem.cursor.1].push('.');
    }

    if mem.cursor.0 == 39 {
        mem.cursor.1 += 1;
        mem.cursor.0 = 0;
        mem.display.push(String::new());
    }
    else {
        mem.cursor.0 += 1;
    }
}

pub fn run() {
    let lines = fs::read_to_string("input/day10in.txt").expect("Failed to read file!");
    let mut mem = Memory::new();


    for line in lines.split('\n') {
        match line {
            "noop" => noop(&mut mem),
            _ if line.starts_with("addx ") => {
                let val: isize = line.split(' ').skip(1).take(1).collect::<String>().parse().unwrap();
                addx(&mut mem, val);
            },
            _ => panic!("Invalid instruction!"),
        }
    }
    
    println!("{}", mem.clocks.iter().take(6).sum::<isize>());
    println!("{}", mem.display.join("\n"));
}
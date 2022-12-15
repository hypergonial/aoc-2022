use std::{fs, collections::HashSet};
use rayon::prelude::*;

const ROW: isize = 2000000;
const UPPER_BOUND: isize = 4000000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Coordinate {
    x: isize,
    y: isize,
}

impl Coordinate {
    fn new(x: isize, y: isize) -> Self {
        Coordinate { x, y }
    }

    #[inline]
    fn get_manhattan_distance(&self, other: &Coordinate) -> usize {
        ((self.x-other.x).abs() + (self.y-other.y).abs()) as usize
    }
}

#[derive(Debug)]
struct Sensor {
    pos: Coordinate,
    range: usize,
}

impl Sensor {
    fn new(pos: &Coordinate, beacon: &Coordinate) -> Self {
        let range = pos.get_manhattan_distance(beacon);
        
        Sensor {pos: *pos, range: range as usize}
    }

    fn get_bounds(&self) -> Vec<Coordinate> {
        let b = (self.range+1) as isize;

        (0..b).flat_map(|step| {
            vec![
                Coordinate::new(self.pos.x+step, self.pos.y+step-b),
                Coordinate::new(self.pos.x+b-step, self.pos.y+step),
                Coordinate::new(self.pos.x-step, self.pos.y+b-step),
                Coordinate::new(self.pos.x+step-b, self.pos.y-step),
            ]
        }).collect()
    }
}

pub fn run() {
    let lines = fs::read_to_string("input/day15in.txt").expect("Failed to read file!");
    // Set of beacons & scanners
    let mut devices: HashSet<Coordinate> = HashSet::new();
    let mut sensors: Vec<Sensor> = Vec::new(); 

    for line in lines.split('\n') {
        let trunc = line
            .replace("Sensor at x=", "")
            .replace(", y=", " ")
            .replace(": closest beacon is at x=", " ")
            .replace(", y=", " ");
        let nums = trunc.split(' ').map(|n| n.parse::<isize>().unwrap()).collect::<Vec<isize>>();
        
        let sensor = Coordinate::new(nums[0], nums[1]);
        let beacon = Coordinate::new(nums[2], nums[3]);
        devices.insert(sensor); devices.insert(beacon);
        sensors.push(Sensor::new(&sensor, &beacon));
        
    }

    // Part 1
    let min_x = devices.iter().map(|c| c.x).min().unwrap()*2;
    let max_x = devices.iter().map(|c| c.x).max().unwrap()*2;

    let count: usize = (min_x..=max_x).into_par_iter().map(|x| {
        let current = Coordinate::new(x, ROW);

        if !devices.contains(&current) && sensors.par_iter().any(|s| s.pos.get_manhattan_distance(&current) <= s.range) {
            return 1
        }
        0
    }).sum();

    println!("{}", count);

    // Part 2
    let freq: usize = sensors.par_iter().try_for_each(|s| {
        s.get_bounds().par_iter().try_for_each(|curr| {
            if curr.x < 0 
            || curr.x > UPPER_BOUND 
            || curr.y < 0 
            || curr.y > UPPER_BOUND 
            || sensors.iter().any(|o| o.pos.get_manhattan_distance(curr) <= o.range) {
                return Ok(());
            }
            Err((curr.x*4000000+curr.y) as usize)
        })
    }).unwrap_err();

    println!("{}", freq);
}

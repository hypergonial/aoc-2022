use std::{fs, collections::VecDeque, hash::Hash};
use rayon::prelude::*;

const ABC: &str = "abcdefghijklmnopqrstuvwxyz";

#[derive(Debug, Clone, Copy, PartialOrd, Ord, Default)]
struct Coordinate {
    x: isize,
    y: isize,
    height: isize,
    distance_from_start: Option<isize>,
    visited: bool,
}

impl Hash for Coordinate {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.height.hash(state);
    }
}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.height == other.height
    }
}
impl Eq for Coordinate {}

impl Coordinate {
    fn new(x: isize, y: isize, height: isize) -> Self {
        Coordinate { x, y, height, distance_from_start: None, visited: false }
    }

    #[inline]
    /// Get the distance between two coordinates. Uses taxicab distance.
    fn get_distance(&self, other: &Coordinate) -> isize {
        (self.x-other.x).abs()+(self.y-other.y).abs()
    }

    /// https://en.wikipedia.org/wiki/Breadth-first_search
    fn get_path_length(&mut self, dest: &Coordinate, mut map: Vec<Coordinate>) -> Option<isize> {
        let mut queue: VecDeque<Coordinate> = VecDeque::new();
        self.visited = true;
        queue.push_front(*self);

        while let Some(coord) = queue.pop_front() {
            if &coord == dest {
                return coord.distance_from_start;
            }

            for mut neighbour in coord.get_neighbouring(&mut map) {
                neighbour.visited = true;
                neighbour.distance_from_start = Some(coord.distance_from_start.unwrap()+1);
                queue.push_back(*neighbour);
            }
        }
        None
    }

    /// Get neighbouring valid coordinates, filters out coordinates that are invalid for path.
    fn get_neighbouring<'a>(&self, map: &'a mut Vec<Coordinate>) -> Vec<&'a mut Coordinate> {
        map.iter_mut().filter(|c| {
            self.get_distance(c) == 1 && c.height - self.height <= 1 && !c.visited
        }).collect::<Vec<&mut Coordinate>>()
    }
}

pub fn run() {
    let lines = fs::read_to_string("input/day12in.txt").expect("Failed to read file!");
    let mut map: Vec<Coordinate> = Vec::new();
    let mut start: Coordinate = Coordinate::default();
    let mut finish: Coordinate = Coordinate::default();

    for (i, line) in lines.split('\n').enumerate() {
        map.extend(line.chars().enumerate().map(|(j, c)| {
            match c {
                'S' => {
                    start = Coordinate::new(j as isize, i as isize, ABC.find('a').unwrap() as isize);
                    start.distance_from_start = Some(0);
                    start
                },
                'E' => {
                    finish = Coordinate::new(j as isize, i as isize, ABC.find('z').unwrap() as isize);
                    finish
                },
                x => Coordinate::new(j as isize, i as isize, ABC.find(x).expect("Invalid character found!") as isize),
            }
        }));
    }

    //Task 1
    println!("{}", start.get_path_length(&finish, map.clone()).expect("No path!"));

    // Task 2
    let starts = map.iter_mut().filter(|c| c.height == 0).map(|c| *c).collect::<Vec<Coordinate>>();

    // Use rayon for multithreading performance gains
    // Roughly takes ~20 seconds to execute in debug mode, ~2 seconds in release
    let shortest = starts.par_iter().map(|s| {
        let mut c = *s;
        c.distance_from_start = Some(0);
        c.get_path_length(&finish, map.clone())
    }).flatten().min().unwrap();

    println!("{}", shortest);

}
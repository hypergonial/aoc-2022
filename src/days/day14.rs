use std::{fs, collections::HashSet};


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn new(x: usize, y: usize) -> Self {
        Coordinate { x, y }
    }
    fn parse(text: &str) -> Self {
        let coords = text.split(',').collect::<Vec<&str>>();
        Coordinate { x: coords[0].parse().unwrap(), y: coords[1].parse().unwrap() }
    }
    /// Generate a straight line between two points
    fn generate_line(&self, other: &Coordinate) -> HashSet<Coordinate> {
        let mut coords: HashSet<Coordinate> = HashSet::new();

        if self.x > other.x {
            for x in other.x..=self.x {
                coords.insert(Coordinate::new(x, self.y));
            }
        }
        else {
            for x in self.x..=other.x {
                coords.insert(Coordinate::new(x, self.y));
            }
        }

        if self.y > other.y {
            for y in other.y..=self.y {
                coords.insert(Coordinate::new(self.x, y));
            }
        }
        else {
            for y in self.y..=other.y {
                coords.insert(Coordinate::new(self.x, y));
            }
        }
        coords
    }

    /// Make the object fall according to the specified ruleset
    /// ## Returns:
    /// If the object settled successfully or not.
    fn fall(mut self, map: &mut HashSet<Coordinate>, floor: usize) -> bool {
        loop {
            if map.contains(&Coordinate::new(self.x, self.y+1)) {
                if map.contains(&Coordinate::new(self.x-1, self.y+1)) {
                    if map.contains(&Coordinate::new(self.x+1, self.y+1)) {
                        map.insert(self);
                        if self.y == 0 {
                            return false;
                        }
                        return true;
                    }
                    else {
                        self.y += 1;
                        self.x += 1;
                    }
                }
                else {
                    self.y += 1;
                    self.x -= 1;
                }
            }
            else {
                self.y += 1;
            }

            if self.y == floor-1 {
                map.insert(self);
                return true;
            }
        }
    }
    
}


pub fn run() {
    let lines = fs::read_to_string("input/day14in.txt").expect("Failed to read file!");
    let mut map: HashSet<Coordinate> = HashSet::new();

    for line in lines.split('\n') {
        let coords = line.split(" -> ").collect::<Vec<&str>>();
        for i in 1..coords.len() {
            let (left, right) = (Coordinate::parse(coords[i-1]), Coordinate::parse(coords[i]));
            map.extend(left.generate_line(&right))
        }
    }
    let floor = map.iter().max_by(|a, b| a.y.cmp(&b.y)).unwrap().y+2;
    let mut counter = 0;
    while Coordinate::new(500, 0).fall(&mut map, floor) {
        counter += 1;
    }
    println!("{:#?}", counter+1);
}
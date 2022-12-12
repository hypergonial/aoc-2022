use std::fs;

const ABC: &str = "abcdefghijklmnopqrstuvwxyz";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
struct Coordinate {
    x: isize,
    y: isize,
    height: usize,
}

impl Coordinate {
    fn new(x: isize, y: isize, height: usize) -> Self {
        Coordinate { x, y, height }
    }

    #[inline]
    /// Get the distance between two coordinates. Uses taxicab distance.
    fn get_distance(&self, other: &Coordinate) -> isize {
        (self.x-other.x).abs()+(self.y-other.y).abs()
    }

    fn get_path_length(&self, finish: &Coordinate, map: Vec<Vec<Coordinate>>) -> isize {
        
        unimplemented!()
    }

    /// Get neighbouring valid coordinates, filters out coordinates that are invalid for path.
    fn get_neighbouring<'a>(&self, map: &'a Vec<Vec<Coordinate>>) -> Vec<&'a Coordinate> {
        map.iter().map(|l| l.iter().filter(|c| {
            (self.x-c.x).abs().max((self.y-c.y).abs()) <= 1 && c.height - self.height <= 1
        })).flatten().collect::<Vec<&Coordinate>>()
    }
}

fn find_best<'a>(current: &Coordinate, finish: &Coordinate, map: &'a Vec<Vec<Coordinate>>) -> &'a Coordinate {
    let mut neighbours = current.get_neighbouring(map);
    neighbours.sort_by(|a, b| a.get_distance(finish).cmp(&b.get_distance(finish)));
    neighbours[0]
}

pub fn run() {
    let lines = fs::read_to_string("input/day12in.txt").expect("Failed to read file!");
    let mut map: Vec<Vec<Coordinate>> = Vec::new();
    let mut start: Coordinate = Coordinate::default();
    let mut finish: Coordinate = Coordinate::default();

    for (i, line) in lines.split('\n').enumerate() {
        map.push(line.chars().enumerate().map(|(j, c)| {
            match c {
                'S' => {
                    start = Coordinate::new(j as isize, i as isize, ABC.find('a').unwrap());
                    start
                },
                'E' => {
                    finish = Coordinate::new(j as isize, i as isize, ABC.find('a').unwrap());
                    finish
                },
                x => Coordinate::new(j as isize, i as isize, ABC.find(x).expect("Invalid character found!")),
            }
        }).collect());
    }

    println!("{:#?}", map);
    println!("{:?} {:?}", start, finish);
}
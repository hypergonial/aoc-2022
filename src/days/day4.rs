use std::fs;

fn contains_all((a, b): (u32, u32), (c, d): (u32, u32)) -> bool {
    (a..=b).all(|x| (c..=d).contains(&x)) || (c..=d).all(|x| (a..=b).contains(&x))
}

fn contains_any((a, b): (u32, u32), (c, d): (u32, u32)) -> bool {
    (a..=b).any(|x| (c..=d).contains(&x)) || (c..=d).any(|x| (a..=b).contains(&x))
}

pub fn run() {
    let lines = fs::read_to_string("input/day4in.txt").expect("Failed to read file!");
    let mut count_all = 0;
    let mut count_any = 0;

    for line in lines.split('\n') {
        let mut ranges = line.split(',');
        let fst: Vec<u32> = ranges.next().unwrap().split('-').map(|x| x.parse::<u32>().unwrap()).collect();
        let snd: Vec<u32> = ranges.next().unwrap().split('-').map(|x| x.parse::<u32>().unwrap()).collect();

        if contains_all((fst[0], fst[1]), (snd[0], snd[1])) {
            count_all += 1;
        }
        if contains_any((fst[0], fst[1]), (snd[0], snd[1])) {
            count_any += 1;
        }
    }

    println!("{count_all} {count_any}");

}
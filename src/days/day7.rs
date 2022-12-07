use std::{fs, collections::HashMap};

#[derive(Debug)]
enum FSObject {
    File(FileData),
    Directory(DirectoryData),
    PartialDirectory(String),
}

#[derive(Debug)]
struct DirectoryData {
    name: String,
    path: String,
    contents: Vec<FSObject>
}

impl DirectoryData {
    fn size(&self) -> usize {
        let mut dir_size = 0;
        for item in self.contents.iter() {
            match item {
                FSObject::File(file) => dir_size += file.size,
                FSObject::Directory(dir) => dir_size += dir.size(),
                FSObject::PartialDirectory(_) => panic!("Trying to calculate size of partial directory record!"),
            }
        }
        dir_size
    }

    fn gather_sizes(&self, map: &mut HashMap<String, usize>) {
        let mut dir_size = 0;
        for item in self.contents.iter() {
            match item {
                FSObject::File(file) => dir_size += file.size,
                FSObject::Directory(dir) => {dir.gather_sizes(map); dir_size += dir.size()},
                FSObject::PartialDirectory(_) => panic!("Trying to calculate size of partial directory record!"),
            }
        }
        if map.insert(self.path.clone(), dir_size).is_some() {
            panic!("Multiple directories on same path!");
        }
    }
}

#[derive(Debug)]
struct FileData {
    name: String,
    size: usize,
}

/// Read a directory's contents. Directories inside directories can only be filled in with partial references,
/// since data about them may not be guaranteed to be ready.
fn read_dir_contents(lines: &[&str]) -> Vec<FSObject> {
    let mut contents = Vec::new();

    for line in lines {
        if line.starts_with("dir ") {
            contents.push(FSObject::PartialDirectory(line.split(' ').skip(1).take(1).collect()));
        }
        else {
            let x = line.split(' ').collect::<Vec<&str>>();
            contents.push(FSObject::File(FileData { name: x[1].to_string(), size: x[0].parse().expect("Cannot parse filesize!") }));
        }
    }
    contents
}

/// Navigate from root to path and return the dir contents
/// Returns a mutable reference to the directory to let you modify directory contents.
fn navigate_to_dir<'a>(root: &'a mut DirectoryData, path: &[&str]) -> &'a mut DirectoryData {
    if path[0] != root.name {
        panic!("Path must start at specified root directory!");
    }
    if path.len() == 1 {
        return root
    }

    let nav_to = path[1].to_string();
    let mut next: Option<&mut DirectoryData> = None;

    for entry in &mut root.contents {
        if let FSObject::Directory(data) = entry {
            if data.name == nav_to {
                next = Some(data);
            }
        }
    }
    
    if path.len() == 1 {
        next.expect("Invalid or partial path!")
    }
    else {
        navigate_to_dir(next.expect("Invalid or partial path!"), &path[1..])
    }
}

/// Find a directory from root, and replace a given partial entry with a directory.
/// Mutates 'root' to fill in the missing directory data at 'path'.
fn replace_partial_dir(root: &mut DirectoryData, path: &[&str], data: DirectoryData) {
    let dir = navigate_to_dir(root, path);

    let mut index = None;

    for (i, e) in dir.contents.iter().enumerate() {
        if let FSObject::PartialDirectory(name) = e {
            if name == &data.name {
                index = Some(i);
                break
            }
        }
    }
    dir.contents[index.expect("Cannot find partial entry to replace!")] = FSObject::Directory(data);
}


pub fn run() {
    let file = fs::read_to_string("input/day7in.txt").expect("Failed to read file!");
    let lines = file.split('\n');

    // Construct root folder & path
    let mut path: Vec<&str> = vec!["/"];
    let mut root = DirectoryData {
            name: "/".to_string(),
            contents: read_dir_contents(&lines.clone().skip(2).take_while(|l| !l.starts_with('$')).collect::<Vec<&str>>()),
            path: "/".to_string(),
    };

    // Skip over root folder data
    let mut lines = lines.skip(2).skip_while(|l| !l.starts_with('$'));


    while let Some(l) = lines.next() {
        // Adjust current path
        if l.starts_with("$ cd ") {
            let dir = l.split(' ').last().expect("Missing parameter after command 'cd'!");
            match dir {
                ".." => { path.pop(); },
                x => { path.push(x); },
            }
        }
        // Read data until next command into a new Directory
        else if l == "$ ls" {
            let data = DirectoryData {
                name: path.last().unwrap().to_string(),
                contents: read_dir_contents(&lines.clone().take_while(|l| !l.starts_with('$')).collect::<Vec<&str>>()),
                path: path.clone().join("/"),
            };
            // Replace partial dir reference one folder up with this folder's data
            replace_partial_dir(&mut root, &path[0..path.len()-1], data)
        }
    }

    let mut buf = HashMap::new();
    root.gather_sizes(&mut buf);

    println!("{:?}", buf.iter().map(|(_, s)| s).filter(|size| **size <= 100000).sum::<usize>());

    let free = 70000000-buf["/"];
    println!("{:?}", buf.iter().map(|(_, s)| s).filter(|size| free + size.clone() > 30000000).min().unwrap());   
}

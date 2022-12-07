mod arena_tree;
use std::fs;
use arena_tree::ArenaTree;
use regex::Regex;

fn load_data() -> String {
    fs::read_to_string("data/data.txt").expect("Should have read")
}

struct ElfFile {
    name: String,
    size: usize
}
impl PartialEq for ElfFile {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl ElfFile {
    fn new(name: String, size: usize) -> Self {
        Self { name, size }
    }
}
struct ElfDir {
    name: String,
    size: usize
}
impl PartialEq for ElfDir {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl ElfDir {
    fn new(name: String) -> Self {
        Self { name, size: 0 }
    }
}

#[derive(PartialEq)]
enum FSObject {
    File(ElfFile),
    Dir(ElfDir)
}
impl FSObject {
    fn size(&self) -> usize {
        match self {
            Self::File(f) => f.size,
            Self::Dir(d) => d.size
        }
    }
    fn size_mut(&mut self) -> &mut usize {
        match self {
            Self::File(f) => &mut f.size,
            Self::Dir(d) => &mut d.size
        }
    }
    fn name(&self) -> &String {
        match self {
            Self::File(f) => &f.name,
            Self::Dir(d) => &d.name
        }
    }
}

type FileSystem = ArenaTree<FSObject>;

fn parse_data(data: &String) -> FileSystem {
    let mut lines = data.lines();
    if lines.next().unwrap_or_default() != "$ cd /" {
        panic!("Didn't start with root dir!");
    }

    let mut file_system = FileSystem::new();
    let mut current_dir = file_system.node(FSObject::Dir(ElfDir::new("/".into())));
    let mut current_path = "/".to_string();

    let cd_up_rex = Regex::new(r"^\$ cd \.\.").unwrap();
    let cd_down_rex = Regex::new(r"^\$ cd (.*)").unwrap();
    let file_rex = Regex::new(r"^(\d+) (.*)").unwrap();
    for line in lines {
        match line {
            "$ ls" => {},
            _cd if cd_up_rex.is_match(line) => {
                let nested_size = file_system.arena[current_dir].val.size();
                current_dir = file_system.arena[current_dir].parent.unwrap();
                *file_system.arena[current_dir].val.size_mut() += nested_size;
                current_path = file_system.arena[current_dir].val.name().clone();
            }
            _cd if cd_down_rex.is_match(line) => {
                let down_dir_name = cd_down_rex.captures(line).unwrap().get(1).unwrap().as_str();
                current_path.push_str(&down_dir_name);
                current_path.push('/');
                let down_dir = file_system.node(FSObject::Dir(ElfDir::new(current_path.clone())));
                file_system.arena[current_dir].children.push(down_dir);
                file_system.arena[down_dir].parent = Some(current_dir);
                current_dir = down_dir;
            },
            _file if file_rex.is_match(line) => {
                let file_name = file_rex.captures(line).unwrap().get(2).unwrap().as_str();
                let file_size = file_rex.captures(line).unwrap().get(1).unwrap().as_str().parse::<usize>().unwrap();
                
                let mut file_path = current_path.clone();
                file_path.push_str(&file_name);
                let file = file_system.node(FSObject::File(ElfFile::new(file_path, file_size)));
                file_system.arena[current_dir].children.push(file);
                file_system.arena[file].parent = Some(current_dir);
                *file_system.arena[current_dir].val.size_mut() += file_size;
            }
            _ => {}
        }
    }

    while current_path != "/" {
        let nested_size = file_system.arena[current_dir].val.size();
        current_dir = file_system.arena[current_dir].parent.unwrap();
        *file_system.arena[current_dir].val.size_mut() += nested_size;
        current_path = file_system.arena[current_dir].val.name().clone();
    }
    file_system
}

fn part_one(file_system: &FileSystem) -> usize {
    file_system.arena.iter().filter_map(|node| {
        match &node.val {
            FSObject::File(_) => None,
            FSObject::Dir(dir) => {
                if dir.size > 100000 { None }
                else { Some(dir.size) }
            }
        }
    }).sum()
}

fn part_two(file_system: &FileSystem) -> usize {
    let space_avail = 70000000 - file_system.arena[0].val.size();
    let space_to_clear = 30000000 - space_avail;

    file_system.arena.iter().filter_map(|node| {
        match &node.val {
            FSObject::File(_) => None,
            FSObject::Dir(dir) => {
                if dir.size < space_to_clear { None }
                else { Some(dir.size) }
            }
        }
    }).min().unwrap() 
}

fn main() {
    let data = load_data();
    let file_system = parse_data(&data);

    let total_size = part_one(&file_system);
    println!("Part one: {}", total_size);

    let min_delete = part_two(&file_system);
    println!("Part two: {}", min_delete);
    //for node in file_system.arena {
    //    println!("Index{}, path {}, size {}, inside {:?}, children {:?}", node.idx, node.val.name(), node.val.size(), node.parent, node.children);
    //}
}



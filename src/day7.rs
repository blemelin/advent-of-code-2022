use util::{FromLine, FromLines, read};

mod util;

fn main() {
    let input: Input = read("inputs/day7.txt");
    println!("Part 1 : {}", input.part_1());
    println!("Part 2 : {}", input.part_2());
}

#[derive(Debug)]
struct Input {
    file_system: FileSystem,
}

impl Input {
    fn part_1(&self) -> usize {
        self.file_system
            .files
            .iter()
            .filter(|it| it.is_directory && it.size < 100000)
            .map(|it| it.size)
            .sum()
    }

    fn part_2(&self) -> usize {
        let used_space = self.file_system.size();
        let remaining_space = 70000000 - used_space;

        self.file_system
            .files
            .iter()
            .filter(|it| it.is_directory && remaining_space + it.size > 30000000)
            .map(|it| it.size)
            .min()
            .unwrap_or(0)
    }
}

#[derive(Debug)]
struct FileSystem {
    current: usize,
    files: Vec<FileSystemEntry>,
}

impl FileSystem {
    fn new() -> Self {
        Self {
            current: 0,
            files: vec![
                FileSystemEntry {
                    name: "/".into(),
                    size: 0,
                    is_directory: true,
                    parent: None,
                }
            ],
        }
    }

    fn current(&self) -> &FileSystemEntry {
        &self.files[self.current]
    }

    fn find_directory(&self, name: &str) -> Option<usize> {
        self.files
            .iter()
            .position(|it| it.parent == Some(self.current) && it.is_directory && it.name == name)
    }

    fn size(&self) -> usize {
        self.files[0].size
    }

    fn navigate(&mut self, path: &str) {
        self.current = match path {
            "/" => 0,
            ".." => self.current().parent.expect("parent should exist when navigating up"),
            name => self.find_directory(name).expect("directory should exist before navigating to it"),
        };
    }

    fn add_file(&mut self, name: &str, size: usize) {
        // Add file
        self.files.push(FileSystemEntry {
            name: name.into(),
            size,
            is_directory: false,
            parent: Some(self.current),
        });

        // Update parents sizes
        let mut parent = Some(self.current);
        while let Some(index) = parent {
            let current = &mut self.files[index];
            current.size += size;
            parent = current.parent;
        }
    }

    fn add_dir(&mut self, name: &str) {
        self.files.push(FileSystemEntry {
            name: name.into(),
            size: 0,
            is_directory: true,
            parent: Some(self.current),
        });
    }
}

#[derive(Debug, Clone)]
struct FileSystemEntry {
    name: String,
    size: usize,
    is_directory: bool,
    parent: Option<usize>,
}

#[derive(Debug)]
enum HistoryLine {
    Ls,
    Cd(String),
    Directory(String),
    File(String, usize),
}

impl FromLines for Input {
    fn from_lines(lines: &[&str]) -> Self {
        let mut file_system = FileSystem::new();

        for line in lines.iter().map(line_to!(HistoryLine)) {
            match line {
                HistoryLine::Cd(name) => file_system.navigate(&name),
                HistoryLine::File(name, size) => file_system.add_file(&name, size),
                HistoryLine::Directory(name) => file_system.add_dir(&name),
                _ => { /* Ignored */ }
            }
        }

        Self {
            file_system
        }
    }
}

impl FromLine for HistoryLine {
    fn from_line(line: &str) -> Self {
        let parts: Vec<&str> = line.split(' ').collect();
        match parts[..] {
            ["$", "ls"] => Self::Ls,
            ["$", "cd", path] => Self::Cd(path.into()),
            ["dir", name] => Self::Directory(name.into()),
            [size, name] => Self::File(name.into(), usize::from_line(size)),
            _ => panic!("{line} is not a valid history line")
        }
    }
}
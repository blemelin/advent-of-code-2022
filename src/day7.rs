use util::{FromLine, FromLines, read};

mod util;

fn main() {
    // Read data
    let Data(file_system) = read("inputs/day7.txt");

    // Part 1
    let result: usize = file_system.files
        .iter()
        .filter(|it| it.parent.is_some() && it.is_directory && it.size < 100000)
        .map(|it| it.size)
        .sum();
    println!("Part 1 : {}", result);

    // Part 2
    let used_space = file_system.size();
    let remaining_space = 70000000 - used_space;
    let result = file_system.files
        .iter()
        .filter(|it| it.is_directory && remaining_space + it.size > 30000000)
        .min_by_key(|it| it.size)
        .expect("filesystem should have a dir large enough to delete")
        .size;
    println!("Part 2 : {}", result);
}

#[derive(Debug)]
struct Data(FileSystem);

impl FromLines for Data {
    fn from_lines(lines: &[&str]) -> Self {
        let mut file_system = FileSystem::new();

        for line in lines.iter().map(line_to!(HistoryLine)) {
            match line {
                HistoryLine::Cd(path) => {
                    file_system.navigate(&path);
                }
                HistoryLine::File(path, size) => {
                    let path = &path;
                    let size = usize::from_line(&size);
                    file_system.add_file(path, size);
                }
                HistoryLine::Directory(path) => {
                    let path = &path;
                    file_system.add_dir(path);
                }
                _ => { /* Ignored */ }
            }
        }

        Self(file_system)
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

    fn find_directory(&self, name : &str) -> Option<usize> {
        self.files
            .iter()
            .enumerate()
            .find(|(_, it)| it.parent == Some(self.current) && it.is_directory && it.name == name)
            .map(|(i, _)| i)
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
    File(String, String),
}

impl FromLine for HistoryLine {
    fn from_line(line: &str) -> Self {
        let mut parts = line.split(' ');
        let start = parts.next().expect("history line should contain at least one part");
        match start {
            "$" => {
                let command = parts.next().expect("history line command should have a name");
                match command {
                    "ls" => {
                        Self::Ls
                    }
                    "cd" => {
                        let path = parts.next().expect("history line should have a path when navigating");
                        Self::Cd(path.into())
                    }
                    _ => panic!("{command} is not a valid command")
                }
            }
            "dir" => {
                let path = parts.next().expect("history line dir should have a path");
                Self::Directory(path.into())
            }
            size => {
                let path = parts.next().expect("history line file should have a path");
                Self::File(path.into(), size.into())
            }
        }
    }
}
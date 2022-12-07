#!/usr/bin/env rust-script

use std::io;
use std::iter::Peekable;
use std::str::FromStr;

#[derive(Debug)]
struct CurrentDirectory<'a> {
    fs: &'a mut FileSystem,
    curr: usize,
}

impl<'a> CurrentDirectory<'a> {
    fn cd(&mut self, name: &str) {
        self.curr = {
            if name == &".."[..] {
                self.fs.directories[self.curr].parent.unwrap()
            } else {
                self.find_directory(name)
            }
        };
    }

    fn add_directory(&mut self, name: String) -> usize {
        self.fs.add_directory(name, Some(self.curr))
    }

    fn add_file(&mut self, size: u64) {
        self.fs.directories[self.curr].files.push(size);
    }

    fn find_directory(&self, name: &str) -> usize {
        for idx in &self.fs.directories[self.curr].directories {
            let dir = &self.fs.directories[*idx];
            if dir.name == name {
                return *idx;
            }
        }

        panic!("Invalid input");
    }
}

#[derive(Default, Debug)]
struct FileSystem {
    directories: Vec<Directory>,
}

impl FileSystem {
    fn add_directory(&mut self, name: String, parent: Option<usize>) -> usize {
        self.directories.push(Directory {
            name,
            parent,
            ..Default::default()
        });

        let idx = self.directories.len() - 1;

        if let Some(parent) = parent {
            self.directories[parent].directories.push(idx);
        }

        idx
    }

    fn directory_size(&self, id: usize) -> u64 {
        let dir = &self.directories[id];
        let mut size = dir.files.iter().sum();

        for subdir in &dir.directories {
            size += self.directory_size(*subdir);
        }

        size
    }
}

#[derive(Default, Debug)]
struct Directory {
    name: String,
    parent: Option<usize>,
    files: Vec<u64>,
    directories: Vec<usize>,
}

fn collect_ls_output<T: Iterator<Item = String>>(
    input: &mut Peekable<T>,
    current_directory: &mut CurrentDirectory,
) {
    loop {
        match input.peek() {
            Some(line) => {
                if line.starts_with("$") {
                    return;
                }
                let line = input.next().unwrap();

                let split: Vec<_> = line.split(" ").collect();
                if line.starts_with("dir") {
                    current_directory.add_directory(split[1].to_string());
                } else {
                    // 123 foo.txt
                    let size = u64::from_str(split[0]).unwrap();
                    current_directory.add_file(size);
                }
            }
            None => return,
        }
    }
}

fn collect_entries<T: Iterator<Item = String>>(
    mut input: &mut Peekable<T>,
    current_directory: &mut CurrentDirectory,
) {
    loop {
        match input.next() {
            Some(line) => {
                if line == "$ ls" {
                    collect_ls_output(&mut input, current_directory);
                } else {
                    // $ cd foo
                    let split: Vec<_> = line.split(" ").collect();
                    current_directory.cd(split[2]);
                }
            }
            None => return,
        }
    }
}

fn size(fs: &FileSystem) -> u64 {
    let total = 70000000;
    let current = total - fs.directory_size(0);
    let mut candidate = total;

    for dir in 1..fs.directories.len() {
        let size = fs.directory_size(dir);
        if current + size >= 30000000 {
            if size < candidate {
                candidate = size;
            }
        }
    }

    candidate
}

fn main() {
    let mut input = io::stdin()
        .lines()
        .skip(1)
        .map(|line| line.unwrap())
        .into_iter()
        .peekable();

    let mut fs = FileSystem::default();
    let root = fs.add_directory("/".to_string(), None);
    let mut current_directory = CurrentDirectory {
        fs: &mut fs,
        curr: root,
    };

    collect_entries(&mut input, &mut current_directory);

    println!("{}", size(&fs));
}

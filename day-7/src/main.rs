use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone)]
struct Entry<'a> {
    name: &'a str,
    size: u64,
    entries: Vec<Rc<RefCell<Entry<'a>>>>,
    parent_dir: Option<Rc<RefCell<Entry<'a>>>>,
}

impl Entry<'_> {
    fn get_size(&self) -> u64 {
        self
            .entries
            .iter()
            .map(|entry| entry.borrow().get_size())
            .sum::<u64>() + self.size
    }
}

fn flatten_dirs(entry: Rc<RefCell<Entry>>) -> Vec<Entry> {
   let mut entries = entry
        .borrow()
        .clone()
        .entries
        .into_iter()
        .filter_map(|e| { if !e.borrow().entries.is_empty() { Some(flatten_dirs(e)) } else { None } }) 
        .flatten()
        .collect::<Vec<Entry>>();

    entries.push(entry.borrow().clone());
    entries
}

fn main() {
    let root = Rc::new(RefCell::new(Entry {
        name: "/",
        size: 0,
        entries: vec![],
        parent_dir: None,
    }));
    let mut current_dir = root.clone();

    for line in include_str!("../input.txt").lines().skip(1) {
        match line.split_whitespace().collect::<Vec<&str>>().as_slice() {
            ["$", "cd", ".."] => {
                let parent_dir = current_dir.borrow().parent_dir.clone();
                current_dir = parent_dir.unwrap();
            }
            ["$", "cd", directory] => {
                let curr = current_dir.borrow().clone();
                let dir = curr.entries.iter().find(|entry| &entry.borrow().name == directory);

                if let Some(next_dir) = dir {
                    current_dir = next_dir.clone();
                } else {
                    let new_dir = Rc::new(RefCell::new(Entry {
                        name: directory,
                        size: 0,
                        entries: vec![],
                        parent_dir: Some(current_dir.clone()),
                    }));

                    current_dir.borrow_mut().entries.push(new_dir.clone());
                    current_dir = new_dir.clone();
                }
            }
            ["$", "ls"] => {
                // ignore
            }
            ["dir", dir_name] => {
                current_dir.borrow_mut().entries.push(Rc::new(RefCell::new(Entry {
                    name: dir_name,
                    size: 0,
                    entries: vec![],
                    parent_dir: Some(current_dir.clone()),
                })));
            }
            [size, file_name] => {
                current_dir.borrow_mut().entries.push(Rc::new(RefCell::new(Entry {
                    name: file_name,
                    size: size.parse::<u64>().unwrap(),
                    entries: vec![],
                    parent_dir: Some(current_dir.clone()),
                })));
            }
            _ => unreachable!(),
        }
    }

    let sum_of_small_dirs = flatten_dirs(root.clone())
        .iter()
        .map(|entry| entry.get_size())
        .filter(|entry| entry <= &100_000)
        .sum::<u64>();

    println!("{sum_of_small_dirs:?}");

    let used_space = root.borrow().get_size();

    let disk_space = 70_000_000;
    let space_required = 30_000_000; 
    let free_space = disk_space - used_space;
    let space_needed = space_required - free_space;

    let min_dir_size_to_remove = flatten_dirs(root.clone())
        .iter()
        .map(|entry| entry.get_size())
        .filter(|entry| entry >= &space_needed)
        .min();

    println!("{min_dir_size_to_remove:?}");
}

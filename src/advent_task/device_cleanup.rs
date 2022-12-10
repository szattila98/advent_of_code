use std::{
    cell::RefCell,
    fmt::Debug,
    rc::{Rc, Weak},
};

use macros::include_str_arr;

use super::AdventTask;

type SizeType = usize;

const FILESYSTEM_SIZE: SizeType = 70000000;
const NEEDED_SPACE: SizeType = 30000000;

// TODO optimization and simplification
// TODO caching
// TODO extract functions
pub struct DeviceCleanup;

impl AdventTask for DeviceCleanup {
    type Solution = SizeType;

    fn get_name(&self) -> &str {
        "Device Cleanup"
    }

    fn get_inputs(&self) -> &[Option<&'static str>] {
        include_str_arr!("./inputs/device_cleanup.txt")
    }

    fn solve_first_part(&self, input: &[Option<&'static str>]) -> Self::Solution {
        let root = Rc::new(RefCell::new(Dir {
            name: "/",
            parent: None,
            children: vec![],
        })) as Rc<RefCell<dyn Node>>;
        let mut current_dir = root.clone();
        for line in input.iter().flatten() {
            match () {
                _ if line.contains("$ ls") => continue,
                _ if line.contains("$ cd") => {
                    let to = line.split(' ').last().unwrap();
                    match to {
                        "/" => {
                            current_dir = root.clone();
                        }
                        ".." => {
                            let parent_dir =
                                current_dir.borrow().get_parent().as_ref().unwrap().clone();
                            current_dir = parent_dir.upgrade().unwrap();
                        }
                        _ => {
                            let to = current_dir.borrow().find_child(to).unwrap().clone();
                            current_dir = to;
                        }
                    }
                }
                _ => {
                    let (first_part, name) = line.split_once(' ').unwrap();
                    if first_part == "dir" {
                        current_dir
                            .borrow_mut()
                            .get_children()
                            .push(Rc::new(RefCell::new(Dir {
                                name,
                                parent: Some(Rc::downgrade(&current_dir)),
                                children: vec![],
                            })) as Rc<RefCell<dyn Node>>)
                    } else {
                        let size = first_part.parse().unwrap();
                        current_dir
                            .borrow_mut()
                            .get_children()
                            .push(Rc::new(RefCell::new(File {
                                name,
                                parent: Some(Rc::downgrade(&current_dir)),
                                size,
                            })) as Rc<RefCell<dyn Node>>)
                    }
                }
            }
        }
        let mut sizes = vec![];
        root.borrow().find_all_sizes(&mut sizes);
        let mut dir_sizes = vec![];
        sizes.iter().for_each(|size| match size {
            Sizes::DirSize(s) => dir_sizes.push(*s),
            _ => (),
        });
        dir_sizes.iter().filter(|size| **size <= 100000).sum()
    }

    fn solve_second_part(&self, input: &[Option<&'static str>]) -> Self::Solution {
        let root = Rc::new(RefCell::new(Dir {
            name: "/",
            parent: None,
            children: vec![],
        })) as Rc<RefCell<dyn Node>>;
        let mut current_dir = root.clone();
        for line in input.iter().flatten() {
            match () {
                _ if line.contains("$ ls") => continue,
                _ if line.contains("$ cd") => {
                    let to = line.split(' ').last().unwrap();
                    match to {
                        "/" => {
                            current_dir = root.clone();
                        }
                        ".." => {
                            let parent_dir =
                                current_dir.borrow().get_parent().as_ref().unwrap().clone();
                            current_dir = parent_dir.upgrade().unwrap();
                        }
                        _ => {
                            let to = current_dir.borrow().find_child(to).unwrap().clone();
                            current_dir = to;
                        }
                    }
                }
                _ => {
                    let (first_part, name) = line.split_once(' ').unwrap();
                    if first_part == "dir" {
                        current_dir
                            .borrow_mut()
                            .get_children()
                            .push(Rc::new(RefCell::new(Dir {
                                name,
                                parent: Some(Rc::downgrade(&current_dir)),
                                children: vec![],
                            })) as Rc<RefCell<dyn Node>>)
                    } else {
                        let size = first_part.parse().unwrap();
                        current_dir
                            .borrow_mut()
                            .get_children()
                            .push(Rc::new(RefCell::new(File {
                                name,
                                parent: Some(Rc::downgrade(&current_dir)),
                                size,
                            })) as Rc<RefCell<dyn Node>>)
                    }
                }
            }
        }
        let total_size = root.borrow().get_size().unwrap();
        let unused_space = FILESYSTEM_SIZE - total_size;
        let mut sizes = vec![];
        root.borrow().find_all_sizes(&mut sizes);
        let mut dir_sizes = vec![];
        sizes.iter().for_each(|size| match size {
            Sizes::DirSize(s) => dir_sizes.push(*s),
            _ => (),
        });
        sizes
            .iter()
            .map(|size| size.unwrap())
            .filter(|size| unused_space + size >= NEEDED_SPACE)
            .min()
            .unwrap()
    }
}

#[derive(Debug)]
enum Sizes {
    DirSize(SizeType),
    FileSize(SizeType),
}

impl Sizes {
    fn unwrap(&self) -> SizeType {
        match self {
            Sizes::DirSize(s) => *s,
            Sizes::FileSize(s) => *s,
        }
    }
}

trait Node<'a>: Debug {
    fn get_name(&self) -> &'a str;
    fn get_parent(&self) -> &Option<Weak<RefCell<dyn Node<'a>>>>;
    fn find_child(&self, name: &'a str) -> Option<&Rc<RefCell<dyn Node<'a>>>>;
    fn get_children(&mut self) -> &mut Vec<Rc<RefCell<dyn Node<'a>>>>;
    fn get_size(&self) -> Sizes;
    fn find_all_sizes(&self, sizes: &mut Vec<Sizes>);
}

#[derive(Debug)]
pub struct Dir<'a> {
    name: &'a str,
    parent: Option<Weak<RefCell<dyn Node<'a>>>>,
    children: Vec<Rc<RefCell<dyn Node<'a>>>>,
}

impl<'a> Node<'a> for Dir<'a> {
    fn get_name(&self) -> &'a str {
        self.name
    }

    fn get_parent(&self) -> &Option<Weak<RefCell<dyn Node<'a>>>> {
        &self.parent
    }

    fn find_child(&self, name: &'a str) -> Option<&Rc<RefCell<dyn Node<'a>>>> {
        self.children
            .iter()
            .find(|child| child.borrow().get_name() == name)
    }

    fn get_children(&mut self) -> &mut Vec<Rc<RefCell<dyn Node<'a>>>> {
        &mut self.children
    }

    fn get_size(&self) -> Sizes {
        let size = self
            .children
            .iter()
            .map(|child| child.borrow().get_size().unwrap())
            .sum();
        Sizes::DirSize(size)
    }

    fn find_all_sizes(&self, sizes: &mut Vec<Sizes>) {
        sizes.push(self.get_size());
        self.children.iter().for_each(|child| {
            child.borrow().find_all_sizes(sizes);
        })
    }
}

#[derive(Debug)]
pub struct File<'a> {
    name: &'a str,
    parent: Option<Weak<RefCell<dyn Node<'a>>>>,
    size: SizeType,
}

impl<'a> Node<'a> for File<'a> {
    fn get_name(&self) -> &'a str {
        self.name
    }

    fn get_parent(&self) -> &Option<Weak<RefCell<dyn Node<'a>>>> {
        &self.parent
    }

    fn find_child(&self, _: &'a str) -> Option<&Rc<RefCell<dyn Node<'a>>>> {
        None
    }

    fn get_children(&mut self) -> &mut Vec<Rc<RefCell<dyn Node<'a>>>> {
        unreachable!()
    }

    fn get_size(&self) -> Sizes {
        Sizes::FileSize(self.size)
    }

    fn find_all_sizes(&self, sizes: &mut Vec<Sizes>) {
        sizes.push(self.get_size());
    }
}

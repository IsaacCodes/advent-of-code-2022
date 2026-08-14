//DEBUG
#![allow(dead_code)]

use std::{cell::RefCell, cmp::min, collections::HashMap, rc::Rc};
use enum_dispatch::enum_dispatch;


//Entry enum
#[enum_dispatch]
pub enum Entry {
    Folder(FolderEntry),
    File(FileEntry),
}

//Properties of all entries
#[enum_dispatch(Entry)]
pub trait EntryProperties {
    fn name(&self) -> &str;
    fn size(&mut self) -> u64;
    fn add_child(&mut self, child: Entry) -> bool;
    fn get_child(&self, name: &str) -> Option<Rc<RefCell<Entry>>>;
    fn solve_1(&mut self) -> u64;
    fn solve_2(&mut self, need_to_free: u64, closest: u64) -> u64;
}



//Folder struct
pub struct FolderEntry {
    name: String,
    children: HashMap<String, Rc<RefCell<Entry>>>,
    size: u64,
}

//Implements properties trait for folder
impl EntryProperties for FolderEntry {
    fn name(&self) -> &str {
        &self.name
    }

    fn size(&mut self) -> u64 {
        //If already calculated
        if self.size != 0 { return self.size }

        //Otherwise calculate
        self.size = self.children.iter()
            .map(|child| child.1.borrow_mut().size())
            .sum();
        
        self.size
    }

    //Add new child (file or folder), always true for Folder
    fn add_child(&mut self, child: Entry) -> bool {
        self.children
            .entry(child.name().to_string())
            .or_insert(Rc::new(RefCell::new(child)));

        true
    }

    //Only returns folders
    fn get_child(&self, name: &str) -> Option<Rc<RefCell<Entry>>> {
        //Clones the Rc from get
        Some(self.children.get(name)?.clone())
    }

    //Returns if meets <= 100000 qualification
    fn solve_1(&mut self) -> u64 {
        let mut res = 0;

        //This folder
        if self.size <= 100000 {
            res += self.size
        }

        //Calculate children
        res += self.children.iter()
            .map(|child| child.1.borrow_mut().solve_1())
            .sum::<u64>();

        res
    }

    //Finds smallest file thats closest to need_to_free
    fn solve_2(&mut self, need_to_free: u64, closest: u64) -> u64 {

        //Ignore anything less than free requirement
        if self.size < need_to_free {
            return closest
        }

        //This size is less than the previous closest
        let this_closest = min(self.size, closest);

        //Call on children
        let children_closest = self.children.iter()
            .map(|child| child.1.borrow_mut().solve_2(need_to_free, this_closest))
            .min().unwrap_or(this_closest);

        min(this_closest, children_closest)
    }
}

//Implements other folder functions
impl FolderEntry {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            children: HashMap::new(),
            size: 0,
        }
    }
}



//File struct
pub struct FileEntry {
    name: String,
    size: u64,
}

//Implements properties trait for file
impl EntryProperties for FileEntry {
    fn name(&self) -> &str {
        &self.name
    }

    fn size(&mut self) -> u64 {
        self.size
    }

    //Cannot add children to a file, always returns false
    fn add_child(&mut self, _: Entry) -> bool {
        false
    }

    //Cannot get children from a file, always returns None
    fn get_child(&self, _: &str) -> Option<Rc<RefCell<Entry>>> {
        None
    }

    //Don't need files for solve
    fn solve_1(&mut self) -> u64 {
        0
    }

    //Don't need files for solve
    fn solve_2(&mut self, _: u64, closest: u64) -> u64 {
        closest
    }
}

//Implements other file functions
impl FileEntry {
    pub fn new(name: &str, size: u64) -> Self {
        Self {
            name: name.to_string(),
            size,
        }
    }
}
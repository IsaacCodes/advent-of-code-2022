use std::{cell::RefCell, collections::HashMap, rc::Rc};
use enum_dispatch::enum_dispatch;

//Entry enum
#[enum_dispatch]
pub enum Entry {
    File(FileEntry),
    Folder(FolderEntry),
}

impl Entry {
    pub fn as_folder(&self) -> Option<&FolderEntry> {
        match self {
            Entry::Folder(folder) => Some(folder),
            Entry::File(_) => None,
        }
    }

    pub fn as_folder_mut(&mut self) -> Option<&mut FolderEntry> {
        match self {
            Entry::Folder(folder) => Some(folder),
            Entry::File(_) => None,
        }
    }
}

//Properties of all entries
#[enum_dispatch(Entry)]
pub trait EntryProperties {
    fn name(&self) -> &str;
    fn size(&mut self) -> u64;
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

    //Mutates to compute size when needed
    fn size(&mut self) -> u64 {
        //If already calculated
        if self.size != 0 { return self.size }

        //Otherwise calculate
        self.size = self.children
            .iter()
            .map(|child| child.1.borrow_mut().size())
            .sum();
        
        self.size
    }
}

impl FolderEntry {
    //Add new child to folder
    pub fn add_child(&mut self, child: Entry) {
        self.children
            .entry(child.name().to_string())
            .or_insert(Rc::new(RefCell::new(child)));
    }

    //Returns child with name
    pub fn get_child(&self, name: &str) -> Option<Rc<RefCell<Entry>>> {
        //Clones the Rc from get
        Some(self.children.get(name)?.clone())
    }

    //Returns if meets <= 100000 qualification
    pub fn solve_1(&mut self) -> u64 {
        let mut res = 0;

        //This folder size
        if self.size <= 100_000 {
            res += self.size;
        }

        //Loop its children
        for child in self.children.values() {
            //If folder, add solve_1
            if let Entry::Folder(folder) = &mut *child.borrow_mut() {
                res += folder.solve_1()
            }
        }

        res
    }

    //Finds smallest file thats closest to need_to_free
    pub fn solve_2(&mut self, need_to_free: u64, closest: u64) -> u64 {
        //Ignore anything less than free requirement
        if self.size < need_to_free {
            return closest
        }

        //If this size is less than the previous closest
        let mut closest = self.size.min(closest);

        //Loop its children
        for child in self.children.values() {
            //Get min for folders
            if let Entry::Folder(folder) = &mut *child.borrow_mut() {
                closest = closest.min(folder.solve_2(need_to_free, closest));
            }
        }

        closest
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

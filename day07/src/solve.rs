use std::{cell::RefCell, fs::File, io::{BufRead, BufReader}, rc::Rc};

use crate::entries::*;

pub fn solve(mode: u8) {
    //Read file into line reading buffer
    let file = File::open("src/input.txt").unwrap();
    let mut lines = BufReader::new(file)
        .lines()
        .map_while(Result::ok);
    
    //Handle root ($ cd /)
    lines.next();
    let root = FolderEntry::new("/");

    //Current path
    let mut cur_path: Vec<Rc<RefCell<Entry>>> = vec![Rc::new(RefCell::new(root.into()))];

    //Loop through lines
    for line in lines {
        //cd command
        if line.starts_with("$ cd") {
            //Assumes ascii, gets name
            let dir = &line[5..];

            //Go up a dir
            if dir == ".." {
                cur_path.pop();
            }
            //Otherwise add to path
            else {
                let child = cur_path
                .last().unwrap()
                .borrow()
                .get_child(dir).unwrap();
                
                cur_path.push(child);

            }
        }

        //ls command
        else if line.starts_with("$ ls") {
            //Does nothing (only care about output)
        }

        //ls output
        else {
            let (dir_or_size, name) = line.split_once(" ").unwrap();
            let parent = cur_path.last().unwrap().clone();

            parent.borrow_mut().add_child(
                //Folder case
                if dir_or_size == "dir" {
                    FolderEntry::new(name).into()
                }
                //File case
                else {
                    FileEntry::new(name, dir_or_size.parse().unwrap()).into()
                }
            );
        }
    }

    //Get root back + set size
    let mut root = cur_path[0].borrow_mut();
    root.size();

    //Second pass for files the fulfill qualifications
    let res = 
        if mode == 1 {
            root.solve_1()
        }
        else {
            let fs_max = 70000000;
            let total_free_needed = 30000000;
            let root_size = root.size();
            let currently_free = fs_max - root_size;
            let need_to_free = total_free_needed - currently_free;

            root.solve_2(need_to_free, root_size)
        };

    println!("{res}");
}

//TODO: use mode for solve, refactor to make cleaner overall
// doesn't work :(
// linked lists in rust are painful
// especially coming from python

use std::{collections::HashMap, io::stdin};

#[derive(Debug, Clone)]
struct File {
    size: u32,
}

#[derive(Debug, Clone)]
struct Folder<'a> {
    files: Vec<File>,
    folders: HashMap<String, &'a Folder<'a>>,
    parent: Option<&'a mut Folder<'a>>,
}

impl Folder<'_> {
    pub fn get_size(&self) -> u32 {
        let folder_total: u32 = self.folders.iter().map(|(_, f)| f.get_size()).sum();
        let files_total: u32 = self.files.iter().map(|f| f.size).sum();

        files_total + folder_total
    }
}

fn main() {
    let mut s = String::new();
    let mut cd = &mut Folder {
        files: vec![],
        folders: HashMap::new(),
        parent: None,
    };

    loop {
        s.clear();
        stdin().read_line(&mut s).unwrap();

        let tokens: Vec<&str> = s.trim().split(' ').collect();

        let &first = tokens.first().expect("First is empty!");
        let &second = tokens.get(1).expect("No commmand!");
        let third = tokens.get(2);
        if first == "$" {
            // new command
            if second == "cd" {
                match third {
                    Some(&"..") => {
                        let mut new_cd = cd.parent.clone();
                        cd = new_cd.expect("No parent!");
                    }
                    Some(&path) => {
                        cd = {
                            if let Some(child) = cd.folders.get_mut(path) {
                                child
                            } else {
                                let new = &mut Folder {
                                    files: vec![],
                                    folders: HashMap::new(),
                                    parent: Some(cd),
                                };
                                cd.folders.insert(path.to_string(), new);
                                new
                            }
                        };
                    }
                    _ => panic!("bad things happened"),
                }
            } else if second == "ls" {
            }
        }
    }
}

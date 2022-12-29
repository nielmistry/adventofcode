use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
};

use nom::{self, error::ParseError};

enum ContentType {
    Folder(u64),
    File,
}

struct Content {
    content_type: ContentType,
    parent: Option<u64>,
    children: Option<Vec<u64>>,
    size: Option<u32>, // if folder, sum of sizes below. if file, size of file
}

struct FolderStructure {
    map: HashMap<u64, Content>,
}

fn get_hash(fully_qualified_path: &str) -> u64 {
    if fully_qualified_path == "/" {
        return 0;
    }
    let mut hasher = DefaultHasher::new();
    fully_qualified_path.hash(&mut hasher);
    hasher.finish()
}

fn get_parent_path(path: &str) -> String {
    let mut sub_paths = path.split("/").collect::<Vec<&str>>();
    sub_paths.pop(); // get rid of the last one
    if path.ends_with("/") {
        // get rid of one more path because this is a folder!
        sub_paths.pop();
    }
    let mut parent_path = sub_paths.join("/");
    parent_path.push('/');
    parent_path
}
impl FolderStructure {
    pub fn new() -> FolderStructure {
        let mut map = HashMap::new();
        map.insert(
            0 as u64,
            Content {
                content_type: ContentType::Folder(0),
                parent: None,
                children: Some(Vec::new()),
                size: None,
            },
        );
        FolderStructure { map }
    }

    pub fn add_content(&mut self, fully_qualified_path: &str, parent_id: u64, content: Content) {
        let parent = self.map.get_mut(&parent_id).unwrap();
        let uid = get_hash(fully_qualified_path);
        parent.children.as_mut().unwrap().push(uid);
        self.map.insert(uid, content);
    }

    fn modify_folders(&mut self, immediate_parent_id: u64, size: u32) {
        let mut parent_id_opt = Some(immediate_parent_id);
        while let Some(parent_id) = parent_id_opt {
            let mut parent = self.map.get_mut(&parent_id).unwrap();
            parent.size = Some(parent.size.unwrap_or(0) + size);
            parent_id_opt = parent.parent;
        }
    }

    pub fn get_parent_id(&mut self, fully_qualified_path: &str) -> Result<u64, bool> {
        let parent_path = get_parent_path(fully_qualified_path);
        Ok(get_hash(&parent_path))
    }

    pub fn add_file(&mut self, fully_qualified_path: &str, size: u32) -> Result<bool, bool> {
        let parent_id = self.get_parent_id(fully_qualified_path)?;

        let child = Content {
            content_type: ContentType::File,
            parent: Some(parent_id),
            size: Some(size),
            children: None,
        };

        self.modify_folders(parent_id, size);
        self.add_content(fully_qualified_path, parent_id, child);
        Ok(true)
    }

    pub fn add_folder(&mut self, fully_qualified_path: &str) -> Result<u64, bool> {
        let mut fqp = String::from(fully_qualified_path);
        if !fqp.ends_with("/") {
            fqp.push('/');
        }
        let folder_key = get_hash(&fqp);
        let parent_id = self.get_parent_id(&fqp)?;

        let child = Content {
            content_type: ContentType::Folder(folder_key),
            parent: Some(parent_id),
            size: None,
            children: Some(Vec::new()),
        };
        self.add_content(&fqp, parent_id, child);
        Ok(folder_key)
    }
}

struct ListedFile<'b> {
    name: &'b str,
    size: u32,
    t: ContentType,
}

enum ParseResult<'a> {
    ChangeDirectory(&'a str),
    ListFiles,
    PrintResult(ListedFile<'a>),
}

fn parse_line(line: &str) -> ParseResult {
    let subwords = line.split_ascii_whitespace().collect::<Vec<&str>>();

    if subwords[0] == "$" {
        // command
        match subwords[1] {
            "cd" => {
                return ParseResult::ChangeDirectory(subwords[2]);
            }
            "ls" => return ParseResult::ListFiles,
            _ => panic!("Unexpected Command!"),
        }
    } else {
        match subwords[0] {
            "dir" => {
                return ParseResult::PrintResult(ListedFile {
                    name: subwords[1],
                    size: 0,
                    t: ContentType::Folder(0),
                });
            }
            _ => {
                if let Ok(size) = subwords[0].parse::<u32>() {
                    return ParseResult::PrintResult(ListedFile {
                        name: subwords[1],
                        size,
                        t: ContentType::File,
                    });
                } else {
                    panic!("Expected a size here!");
                }
            }
        }
    }
}

fn parse_file(input: &str) -> FolderStructure {
    let mut fs = FolderStructure::new();
    let lines = input.lines().collect::<Vec<&str>>();
    let mut current_fqp: String = "/".to_string();

    for line in lines {
        let parsed_line = parse_line(line);
        match parsed_line {
            ParseResult::ChangeDirectory(new_dir) => match new_dir {
                ".." => {
                    let new_path = get_parent_path(&current_fqp);
                    current_fqp = new_path;
                }
                _ => {
                    if new_dir.starts_with("/") {
                        // absolute path
                        current_fqp = new_dir.to_string();
                    } else {
                        // relative path
                        current_fqp = current_fqp + new_dir + "/";
                    }
                }
            },

            ParseResult::ListFiles => {}
            ParseResult::PrintResult(res) => {
                let path = current_fqp.clone() + res.name;
                match res.t {
                    ContentType::File => {
                        fs.add_file(&path, res.size);
                    }
                    ContentType::Folder(_) => {
                        fs.add_folder(&path);
                    }
                }
            }
        }
    }

    fs
}

pub fn part1(input: &str) -> u32 {
    let mut fs = parse_file(input);
    let mut output = 0;

    // tally up any directories with size <= 100000
    for (_key, value) in fs.map {
        match value.content_type {
            ContentType::File => {} //ignore
            ContentType::Folder(_) => {
                if let Some(v) = value.size {
                    if v <= 100000 {
                        output += value.size.unwrap();
                    }
                }
            }
        }
    }

    output
}

pub fn part2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{borrow::Borrow, fs};

    #[test]
    fn tree_test() {
        let mut fs = FolderStructure::new();
        fs.add_file("/t2.log", 10);
        let test = fs.map.get(&0).unwrap();
        assert_eq!(test.size, Some(10));

        let k = fs.add_folder("/test/").unwrap();
        fs.add_file("/test/test.log", 20);

        let test = fs.map.get(&0).unwrap();
        assert_eq!(test.size, Some(30));
        let test2 = fs.map.get(&k).unwrap();
        assert_eq!(test2.size, Some(20));
    }

    #[test]
    fn part1_test() {
        let input = fs::read_to_string("test.txt").unwrap();
        assert_eq!(part1(&input), 95437);
    }

    // #[test]
    // fn part2_test() {
    //     let input = fs::read_to_string("test.txt").unwrap();
    //     let ans = part2(&input);
    //     assert_eq!("MCD", ans);
    // }
}

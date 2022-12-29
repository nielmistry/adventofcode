use rand::Rng;
use std::collections::HashMap;

enum ContentType {
    Folder(u32),
    File,
}

struct Content {
    content_type: ContentType,
    parent: Option<u32>,
    children: Option<Vec<u32>>,
    size: Option<u32>, // if folder, sum of sizes below. if file, size of file
}

struct FolderStructure {
    map: HashMap<u32, Content>,
}

fn generate_uid() -> u32 {
    0
}
impl FolderStructure {
    pub fn new() -> FolderStructure {
        let mut map = HashMap::new();
        map.insert(
            0 as u32,
            Content {
                content_type: ContentType::Folder(0),
                parent: None,
                children: Some(Vec::new()),
                size: None,
            },
        );
        FolderStructure { map }
    }

    pub fn add_content(&mut self, parent_id: u32, content: Content) {
        let parent = self.map.get_mut(&parent_id).unwrap();
        let uid = generate_uid();
        parent.children.as_mut().unwrap().push(uid);
        self.map.insert(uid, content);
    }

    fn modify_folders(&mut self, immediate_parent_id: u32, size: u32) {
        let mut parent_id_opt = Some(immediate_parent_id);
        while let Some(parent_id) = parent_id_opt {
            let mut parent = self.map.get_mut(&parent_id).unwrap();
            parent.size = Some(parent.size.unwrap_or(0) + size);
            parent_id_opt = parent.parent;
        }
    }

    pub fn add_file(&mut self, parent_id: u32, size: u32) {
        let child = Content {
            content_type: ContentType::File,
            parent: Some(parent_id),
            size: Some(size),
            children: None,
        };

        self.modify_folders(parent_id, size);
        self.add_content(parent_id, child);
    }

    pub fn add_folder(&mut self, parent_id: u32) {
        let folder_key = generate_uid();
        let child = Content {
            content_type: ContentType::Folder(folder_key),
            parent: Some(parent_id),
            size: None,
            children: Some(Vec::new()),
        };
        self.add_content(parent_id, child);
    }
}

pub fn part1(input: &str) -> usize {
    0
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
        fs.add_file(0, 10);
        let test = fs.map.get(&0).unwrap();
        assert_eq!(test.size, Some(10));

        fs.add_folder(0);
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

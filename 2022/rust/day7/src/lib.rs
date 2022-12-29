use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
};

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
        let mut sub_paths = fully_qualified_path.split("/").collect::<Vec<&str>>();
        // if sub_paths.len() <= 1 {
        //     // We must be in the "/" case:
        //     return Ok(get_hash(&"/"));
        // }
        sub_paths.pop(); // get rid of the last one
        if fully_qualified_path.ends_with("/") {
            // get rid of one more path because this is a folder!
            sub_paths.pop();
        }
        let mut parent_path = sub_paths.join("/");
        parent_path.push('/');
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
        let folder_key = get_hash(fully_qualified_path);
        let parent_id = self.get_parent_id(fully_qualified_path)?;

        let child = Content {
            content_type: ContentType::Folder(folder_key),
            parent: Some(parent_id),
            size: None,
            children: Some(Vec::new()),
        };
        self.add_content(fully_qualified_path, parent_id, child);
        Ok(folder_key)
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

    // #[test]
    // fn part1_test() {
    //     let input = fs::read_to_string("test.txt").unwrap();
    //     assert_eq!(part1(&input), 95437);
    // }

    // #[test]
    // fn part2_test() {
    //     let input = fs::read_to_string("test.txt").unwrap();
    //     let ans = part2(&input);
    //     assert_eq!("MCD", ans);
    // }
}

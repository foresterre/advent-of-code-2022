use camino::{Utf8Path, Utf8PathBuf};
use std::collections::HashMap;

#[derive(Debug)]
pub struct FileSystem {
    // Maps a folder, to the total size of files in the folder.
    // We don't care about files themselves for the puzzle =).
    fs: HashMap<Utf8PathBuf, u32>,
}

impl FileSystem {
    pub fn from_str(input: &str) -> Self {
        let mut current = Utf8PathBuf::from("/");

        input.lines().fold(FileSystem::default(), |mut acc, line| {
            let (lhs, rhs) = line.split_once(' ').unwrap();

            if lhs == "$" && rhs.starts_with("cd") {
                let dir = &rhs[3..];

                if dir == ".." {
                    current.pop();
                } else {
                    current.push(dir);
                }
            } else if let Ok(size) = lhs.parse::<u32>() {
                for ancestor in current.ancestors() {
                    *acc.fs.entry(ancestor.to_path_buf()).or_default() += size;
                }
            }

            acc
        })
    }

    pub fn free_space(&self) -> u32 {
        70000000 - self.space_taken()
    }

    pub fn space_taken(&self) -> u32 {
        self.fs.get(Utf8Path::new("/")).copied().unwrap_or_default()
    }

    pub fn filter_size(&self, f: impl Fn(&&u32) -> bool) -> impl Iterator<Item = &u32> {
        self.fs.values().filter(f)
    }
}

impl Default for FileSystem {
    fn default() -> Self {
        Self {
            fs: Default::default(),
        }
    }
}

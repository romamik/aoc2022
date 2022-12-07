use anyhow::bail;
use anyhow::Result;
use itertools::Itertools;
use std::collections::hash_map;
use std::collections::HashMap;

#[derive(Debug)]
pub enum FSEntry {
    File(File),
    Dir(Dir),
}

impl FSEntry {
    pub fn size(&self) -> usize {
        match self {
            FSEntry::File(File { size }) => *size,
            FSEntry::Dir(dir) => dir.size(),
        }
    }

    pub fn file(&self) -> Result<&File> {
        match self {
            FSEntry::File(file) => Ok(file),
            FSEntry::Dir(_) => bail!("not a file"),
        }
    }

    pub fn file_mut(&mut self) -> Result<&mut File> {
        match self {
            FSEntry::File(file) => Ok(file),
            FSEntry::Dir(_) => bail!("not a file"),
        }
    }

    pub fn dir(&self) -> Result<&Dir> {
        match self {
            FSEntry::Dir(dir) => Ok(dir),
            FSEntry::File(_) => bail!("not a dir"),
        }
    }

    pub fn dir_mut(&mut self) -> Result<&mut Dir> {
        match self {
            FSEntry::Dir(dir) => Ok(dir),
            FSEntry::File(_) => bail!("not a dir"),
        }
    }
}

#[derive(Debug)]
pub struct File {
    size: usize,
}

#[derive(Debug)]
pub struct Dir {
    entries: HashMap<String, FSEntry>,
}

impl Dir {
    pub fn new() -> Dir {
        Dir {
            entries: HashMap::new(),
        }
    }

    pub fn size(&self) -> usize {
        self.entries.iter().map(|(_name, entry)| entry.size()).sum()
    }

    pub fn entries(&self) -> impl Iterator<Item = (&String, &FSEntry)> + '_ {
        self.entries
            .iter()
            .sorted_by(|(name_a, _), (name_b, _)| Ord::cmp(name_a, name_b))
    }

    pub fn _entries_mut(&mut self) -> impl Iterator<Item = (&String, &mut FSEntry)> + '_ {
        self.entries.iter_mut()
    }

    pub fn _entry(&self, name: &str) -> Option<&FSEntry> {
        self.entries.get(name)
    }

    pub fn entry_mut(&mut self, name: &str) -> Option<&mut FSEntry> {
        self.entries.get_mut(name)
    }

    pub fn dir_mut(&mut self, name: &str) -> Option<&mut Dir> {
        self.entry_mut(name).and_then(|e| e.dir_mut().ok())
    }

    fn add_entry(&mut self, name: &str, entry: FSEntry) -> Result<&mut FSEntry> {
        Ok(match self.entries.entry(name.to_string()) {
            hash_map::Entry::Occupied(_) => bail!("already has entry {:?}", name),
            hash_map::Entry::Vacant(e) => e.insert(entry),
        })
    }

    pub fn add_file(&mut self, name: &str, size: usize) -> Result<&mut File> {
        self.add_entry(name, FSEntry::File(File { size }))?
            .file_mut()
    }

    pub fn add_dir(&mut self, name: &str) -> Result<&mut Dir> {
        self.add_entry(name, FSEntry::Dir(Dir::new()))?.dir_mut()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_main() -> Result<()> {
        let mut root = Dir::new();

        root.add_file("some_other_file", 200)?;
        let foo = root.add_dir("foo")?;
        foo.add_file("some_file", 100)?;

        assert_eq!(100, foo.size());
        assert_eq!(300, root.size());

        root.add_file("one_more_file", 300)?;
        assert_eq!(600, root.size());

        let foo = root.dir_mut("foo").unwrap();
        let bar = foo.add_dir("bar")?;
        bar.add_file("file_in_bar", 600)?;

        assert_eq!(100 + 600, foo.size());
        assert_eq!(1200, root.size());

        Ok(())
    }
}

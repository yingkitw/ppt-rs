//! OPC Package handling

use crate::exc::Result;
use std::collections::HashMap;
use std::io::Read;
use std::path::Path;

/// Represents an OPC package (ZIP file)
pub struct Package {
    /// Package parts stored as (path, content)
    parts: HashMap<String, Vec<u8>>,
}

impl Package {
    /// Create a new empty package
    pub fn new() -> Self {
        Package {
            parts: HashMap::new(),
        }
    }

    /// Open a package from a file path
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let file = std::fs::File::open(path)?;
        Self::open_reader(file)
    }

    /// Open a package from a reader
    pub fn open_reader<R: Read + std::io::Seek>(reader: R) -> Result<Self> {
        let mut archive = zip::ZipArchive::new(reader)?;

        let mut parts = HashMap::new();

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;

            if !file.is_dir() {
                let mut content = Vec::new();
                file.read_to_end(&mut content)?;
                parts.insert(file.name().to_string(), content);
            }
        }

        Ok(Package { parts })
    }

    /// Save the package to a file
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let path = path.as_ref();
        let file = std::fs::File::create(path)?;
        self.save_writer(file)
    }

    /// Save the package to a writer
    pub fn save_writer<W: std::io::Write + std::io::Seek>(&self, writer: W) -> Result<()> {
        let mut archive = zip::ZipWriter::new(writer);

        for (path, content) in &self.parts {
            let options = zip::write::FileOptions::default();
            archive.start_file(path, options)?;
            std::io::Write::write_all(&mut archive, content)?;
        }

        archive.finish()?;

        Ok(())
    }

    /// Get a part by path
    pub fn get_part(&self, path: &str) -> Option<&[u8]> {
        self.parts.get(path).map(|v| v.as_slice())
    }

    /// Add or update a part
    pub fn add_part(&mut self, path: String, content: Vec<u8>) {
        self.parts.insert(path, content);
    }

    /// Remove a part by path
    pub fn remove_part(&mut self, path: &str) -> Option<Vec<u8>> {
        self.parts.remove(path)
    }

    /// Check if a part exists
    pub fn has_part(&self, path: &str) -> bool {
        self.parts.contains_key(path)
    }

    /// Get all part paths
    pub fn part_paths(&self) -> Vec<&str> {
        self.parts.keys().map(|s| s.as_str()).collect()
    }

    /// Get number of parts
    pub fn part_count(&self) -> usize {
        self.parts.len()
    }

    /// Get mutable reference to part content
    pub fn get_part_mut(&mut self, path: &str) -> Option<&mut Vec<u8>> {
        self.parts.get_mut(path)
    }

    /// Get part as string (for XML parts)
    pub fn get_part_string(&self, path: &str) -> Option<String> {
        self.parts
            .get(path)
            .map(|v| String::from_utf8_lossy(v).to_string())
    }
}

impl Default for Package {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_package_creation() {
        let package = Package::new();
        assert_eq!(package.part_count(), 0);
    }

    #[test]
    fn test_add_part() {
        let mut package = Package::new();
        package.add_part("test.txt".to_string(), b"content".to_vec());
        assert_eq!(package.part_count(), 1);
        assert_eq!(package.get_part("test.txt"), Some(b"content".as_slice()));
    }

    #[test]
    fn test_part_paths() {
        let mut package = Package::new();
        package.add_part("file1.txt".to_string(), b"content1".to_vec());
        package.add_part("file2.txt".to_string(), b"content2".to_vec());
        let paths = package.part_paths();
        assert_eq!(paths.len(), 2);
    }
}

//! Deduplicate embedded slide media by byte content.

/// Assigns stable 1-based `imageN` part numbers, reusing prior entries for identical bytes.
#[derive(Default)]
pub struct MediaRegistry {
    entries: Vec<(Vec<u8>, String)>,
}

impl MediaRegistry {
    pub fn image_number(&mut self, bytes: &[u8], ext: &str) -> usize {
        if let Some(num) = self.lookup_number(bytes, ext) {
            return num;
        }
        self.entries.push((bytes.to_vec(), ext.to_string()));
        self.entries.len()
    }

    pub fn lookup_number(&self, bytes: &[u8], ext: &str) -> Option<usize> {
        self.entries
            .iter()
            .position(|(existing, existing_ext)| existing.as_slice() == bytes && existing_ext == ext)
            .map(|i| i + 1)
    }

    pub fn files(&self) -> &[(Vec<u8>, String)] {
        &self.entries
    }

    pub fn extensions(&self) -> Vec<String> {
        self.entries.iter().map(|(_, ext)| ext.clone()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deduplicates_identical_image_bytes() {
        let mut registry = MediaRegistry::default();
        let a = registry.image_number(b"same", "png");
        let b = registry.image_number(b"other", "jpg");
        let c = registry.image_number(b"same", "png");
        assert_eq!(a, 1);
        assert_eq!(b, 2);
        assert_eq!(c, 1);
        assert_eq!(registry.files().len(), 2);
    }
}

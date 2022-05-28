use std::fs::File;
use std::path::Path;

trait FileMetaData {
    fn exists(&self) -> bool;
    fn is_writeable(&self) -> bool;
    fn is_readable(&self) -> bool;
}

impl FileMetaData for Path {
    fn exists(&self) -> bool {
        self.is_file()
    }

    fn is_writeable(&self) -> bool {
        if !self.exists() {
            false
        } else {
            File::open(self)
                .map(|f| f.metadata())
                .unwrap()
                .map(|m| !m.permissions().readonly() )
                .unwrap_or(false)
        }
    }

    fn is_readable(&self) -> bool {
        if !self.exists() {
            false
        } else {
            File::open(self).is_ok()
        }
    }
}


fn main() {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn read_only() {
        use std::fs;
        use tempfile;

        let f = tempfile::NamedTempFile::new().unwrap();
        let mut perms = fs::metadata(f.path()).unwrap().permissions();
        perms.set_readonly(true);
        fs::set_permissions(f.path(), perms).unwrap();
        assert_eq!(f.path().is_writeable(), false);

        fs::remove_file(f.path()).unwrap();
    }
    #[test]
    fn writeable() {
        use std::fs;
        use tempfile;

        let f = tempfile::NamedTempFile::new().unwrap();
        assert!(f.path().is_writeable());

        fs::remove_file(f.path()).unwrap();
    }
}
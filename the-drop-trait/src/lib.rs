use std::{
    fs::{self, File},
    path::PathBuf,
};

pub struct TempFile {
    pub path: PathBuf,
}

impl TempFile {
    pub fn new(file_name: impl AsRef<str>) -> Result<Self, String> {
        let file = File::create(file_name.as_ref());

        match file {
            Err(_) => Err("Failed to create temporary file".to_string()),
            Ok(_) => Ok(TempFile {
                path: PathBuf::from(file_name.as_ref()),
            }),
        }
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

pub fn main() {
    let file_path = PathBuf::from("example_temp_file.tmp");
    let tempfile =
        TempFile::new(file_path.to_str().unwrap()).expect("Failed to create temporary file");

    assert!(tempfile.path.exists(), "File does not exist");

    drop(tempfile);

    assert!(!file_path.exists(), "File was not deleted");

    let tempfile_2 = TempFile::new(&String::from("example_temp_file_2.tmp"))
        .expect("Failed to create temporary file");

    drop(tempfile_2);
}

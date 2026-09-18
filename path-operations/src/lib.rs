use std::path::{Path, PathBuf};

pub fn join_paths(base: &str, parts: &[&str]) -> PathBuf {
    let mut path = PathBuf::from(base);

    for part in parts {
        path.push(part);
    }

    path
}

pub fn get_extension(path: &str) -> Option<String> {
    let ext = Path::new(path).extension()?;
    ext.to_str().map(|s| s.to_string())
}

pub fn get_file_name(path: &str) -> Option<String> {
    let file_name = Path::new(path).file_name()?;
    file_name.to_str().map(|s| s.to_string())
}

pub fn get_file_stem(path: &str) -> Option<String> {
    let file_stem = Path::new(path).file_stem()?;
    file_stem.to_str().map(|s| s.to_string())
}

pub fn get_parent(path: &str) -> Option<PathBuf> {
    let path = Path::new(path).parent()?;

    if path.as_os_str().is_empty() {
        return None;
    }

    Some(path.to_path_buf())
}

pub fn change_extension(path: &str, new_ext: &str) -> PathBuf {
    let mut path_buf = PathBuf::from(path);
    path_buf.set_extension(new_ext);
    path_buf
}

pub fn is_absolute(path: &str) -> bool {
    Path::new(path).is_absolute()
}

pub fn normalize_path(path: &str) -> PathBuf {
    PathBuf::from(path)
}

pub fn main() {
    let full_path = join_paths("/home/user", &["documents", "reports"]);
    println!("Joined path: {:?}", full_path);

    let ext = get_extension("document.pdf");
    println!("Extension: {:?}", ext);

    let name = get_file_name("/home/user/document.pdf");
    println!("File name: {:?}", name);

    let parent = get_parent("/home/user/document.pdf");
    println!("Parent: {:?}", parent);

    let new_path = change_extension("report.doc", "pdf");
    println!("New extension: {:?}", new_path);

    println!("Is absolute '/home': {}", is_absolute("/home"));
    println!("Is absolute 'relative': {}", is_absolute("relative"));
}

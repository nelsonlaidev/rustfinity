use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

pub fn to_os_string(s: &str) -> OsString {
    OsString::from(s)
}

pub fn os_str_to_str(os: &OsStr) -> Option<&str> {
    os.to_str()
}

pub fn os_string_to_string_lossy(os: &OsStr) -> String {
    os.to_string_lossy().to_string()
}

pub fn get_file_extension(path: &Path) -> Option<String> {
    path.extension()?.to_str().map(str::to_owned)
}

pub fn join_path_components(components: &[&str]) -> OsString {
    let mut path = PathBuf::new();

    for component in components {
        path.push(component);
    }

    path.into_os_string()
}

pub fn is_valid_utf8(os: &OsStr) -> bool {
    os.to_str().is_some()
}

pub fn main() {
    println!("=== OsString and OsStr Basics ===\n");

    let os = to_os_string("hello.txt");
    println!("to_os_string(\"hello.txt\"): {:?}", os);

    let os_str = OsStr::new("valid_string.rs");
    println!(
        "os_str_to_str(OsStr::new(\"valid_string.rs\")): {:?}",
        os_str_to_str(os_str)
    );

    let os_str = OsStr::new("hello world");
    println!(
        "os_string_to_string_lossy: {}",
        os_string_to_string_lossy(os_str)
    );

    let path = Path::new("document.pdf");
    println!(
        "get_file_extension(Path::new(\"document.pdf\")): {:?}",
        get_file_extension(path)
    );

    let components = &["home", "user", "documents"];
    let joined = join_path_components(components);
    println!("join_path_components(&{:?}): {:?}", components, joined);

    let os_str = OsStr::new("valid utf8 string");
    println!(
        "is_valid_utf8(OsStr::new(\"valid utf8 string\")): {}",
        is_valid_utf8(os_str)
    );
}

use std::path::{Path, PathBuf};

pub fn path_without_last_component(path: &str) -> String {
    let mut p = PathBuf::from(path);
    p.pop();
    p.as_path().to_str().unwrap().to_string()
}

pub fn path_components_num(path: &str) -> usize {
    let p = Path::new(path);
    // '/a/b' has 3 components: '/', 'a', 'b'
    p.components().count()
}

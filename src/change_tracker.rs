use std::fs;
use crate::fs_objects::FsObject;

pub fn parse_json(json: &String) -> FsObject {
    serde_json::from_str(json).unwrap()
}

pub fn update_tracking_file(path: &str, content: &str) {
    fs::write(path, content).expect("Unable to write to file");
}

pub fn load_tracking_file(path: &str) -> String {
    fs::read_to_string(path).expect("Unable to read file")
}

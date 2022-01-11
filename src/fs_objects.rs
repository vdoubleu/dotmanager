use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum FsObject {
    File(File),
    Dir(Dir),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    name: String,
    last_modified: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Dir {
    name: String,
    last_modified: u64,
    children: Vec<FsObject>,
}

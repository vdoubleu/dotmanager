use crate::fs_objects::{FsObject, File, Dir};

// checks if a file or folder has been modified using timestamp
// returns filepath of modified file or folder
pub fn check_change(root_path: &String, last_check_metadata: FsObject) -> Result<Vec<String>> {
    if let Ok(meta) = metadata(root_path) {
        if meta.is_file() {
            // check file modified time against last time we checked
            if let Ok(time) = metadata.modified() {
                if let Ok(epoch_time) = time.duration_since(SystemTime::UNIX_EPOCH) {
                    if epoch_time.as_secs() == last_check_metadata.modified {
                        return Ok(vec![])
                    } else {
                        return Ok(vec![root_path.to_string()])
                    }
                } else {
                    return Err(Error::new(ErrorKind::Other, "Error getting file modified time"));
                }
            } else {
                return Err(Error::new(ErrorKind::Other, "Could not get file metadata"));
            }
        } else if meta.is_dir() {
            // recurse into directory
            if let Ok(time) = metadata.modified() {
                if let Ok(epoch_time) = time.duration_since(SystemTime::UNIX_EPOCH) {
                    if epoch_time.as_secs() == last_check_metadata.modified {
                        return Ok(vec![])
                    } else {
                        return Ok(vec![root_path.to_string()])
                    }
                } else {
                    return Err(Error::new(ErrorKind::Other, "Error getting file modified time"));
                }
            } else {
                return Err(Error::new(ErrorKind::Other, "Could not get file metadata"));
            }
        } else if meta.is_symlink() {
            // check symlink target

        } else {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("Not a file, directory or symlink, what is this: {}", f)));
        }
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::NotFound, "File not found"))
    }
}

use std::io::Error as IoError;
use std::path::PathBuf;

#[derive(Debug)]
pub enum FileError {
    IoError(IoError),
    PathNotFound,
}

impl From<IoError> for FileError {
    fn from(error: IoError) -> FileError {
        FileError::IoError(error)
    }
}

pub fn get_file_path(mnemonic: Option<&str>) -> Result<PathBuf, FileError> {
    let mut path = std::env::current_exe()?
        .parent()
        .ok_or(FileError::PathNotFound)?
        .to_path_buf();

    match mnemonic {
        None => path.push("work"),
        Some(mnemonic) => {
            path.push("tasks");
            path.push(mnemonic);
        }
    }

    path.push("sessions.log");
    Ok(path)
}

#[test]
fn test() {
    assert!(get_file_path(None).unwrap().ends_with("work/sessions.log"));
    assert!(get_file_path(Some("mytask")).unwrap().ends_with("tasks/mytask/sessions.log"));
}

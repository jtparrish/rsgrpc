use std::path::{Path, PathBuf};
use std::io::Result as ioResult;
use std::io::{Error as ioError, ErrorKind as ioErrorKind};
use std::io::{Read, Write};
use std::fs;
use service::{FileName, PutFileMsg, FileContentResponse, Success};

const FS_ROOT: &str = "/home/jt/Programs/Projects/rust-grpc/testbed";

// #[inline(always)]
fn absolute_path(fspath: &str) -> ioResult<PathBuf> {
    let mut string_chars = fspath.chars();
    let first_char = string_chars.next().ok_or(ioError::new(ioErrorKind::Other, "invalid path: could not check first character"))?;
    let fspath = if first_char == '/' {
        string_chars.collect::<String>()
    } else {
        fspath.to_owned()
    };
    let path = Path::new(FS_ROOT).join(fspath);
    let (parent, file_name) = (path.parent().expect("path broken").canonicalize(), path.file_name().expect("path broken: filename"));
    parent.map(
        |mut f| {
            f.push(file_name);
            f
        })
}

#[inline(always)]
fn validate_path(canon_path: &Path) -> bool {
    canon_path.starts_with(FS_ROOT)
}

pub fn create_file(fname: FileName) -> Success {
    let path = absolute_path(&fname.name);
    if !path.is_ok() {
        return Success { success: false };
    };
    let path = path.expect("should not be err");
    
    Success { success: validate_path(path.as_path()) && fs::OpenOptions::new().write(true).create_new(true).open(path).is_ok() }
}

pub fn read_file(fname: FileName) -> FileContentResponse {
    let path = absolute_path(&fname.name);
    if path.is_err() {
        return FileContentResponse { success: false, contents: "bad path".to_owned() };
    };
    let path = path.expect("should not be err");
    let file = fs::OpenOptions::new().read(true).open(path);

    let mut res = FileContentResponse { success: file.is_ok(), contents: "file issue".to_owned() };

    let mut contents = String::default();

    res.success = res.success && file.expect("should not be err").read_to_string(&mut contents).is_ok();

    if res.success {
        res.contents = contents;
    }

    res
}

pub fn write_file(msg: PutFileMsg) -> Success {
    let path = absolute_path(&msg.name);
    if path.is_err() {
        return Success { success: false };
    };
    let path = path.expect("should not be err");
    let file = fs::OpenOptions::new().write(true).open(path);

    return Success { success: file.and_then(|mut f| f.write_all(msg.contents.as_bytes())).is_ok() }
}
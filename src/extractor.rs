use std::fs::create_dir;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

use tempdir::*;
use zip::*;

pub fn extract(path: Path){
    let tmp_dir = TempDir::new("email_extractor").unwrap();
    let item_path = tmp_dir.path().join(path.file_stem());
    let file = zip.by_path();
    let mut reader = Reader::from_reader(BufReader::new(file));
}

use std::fs::File;
use std::io::prelude::*;
use std::io::{Write, BufReader};

use tempdir::*;

pub struct Document {
    pub path: String,
    pub filename: String,
    pub temp_dir: Option<TempDir>,
    pub emails: Option<Vec<String>>
}

pub fn extract(mut document: Document) -> Document {
    let tmp_dir = TempDir::new("email_extractor").unwrap();
    let item_path = tmp_dir.path().join(&document.filename);
    let file = File::open(&document.path).unwrap();
    let reader = BufReader::new(file);
    let mut zip = zip::ZipArchive::new(reader).unwrap();

    for i in 0..zip.len() {
        let mut file = zip.by_index(i).unwrap();
        let mut content_buffer: Vec<u8> = vec!();
        file.read_to_end(&mut content_buffer).unwrap();
        let mut buffer = File::create(item_path.join(file.name())).unwrap();
        buffer.write(content_buffer.as_slice()).unwrap();
    }
    document.temp_dir = Some(tmp_dir);
    document
}

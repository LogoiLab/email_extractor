extern crate regex;
extern crate tempdir;
extern crate zip;

pub mod parser;

extern crate glob;
use self::glob::glob;

pub mod extractor;

use std::fs::OpenOptions;
use std::io::prelude::*;

use self::parser::parse;
use self::extractor::extract;
use self::extractor::Document;

fn main() {
    for entry in glob("*").unwrap() {
        let path = match entry {
            Ok(path) => path,
            _ => continue,
        };
        let filename = String::from(path.file_name().unwrap().to_str().unwrap());
        if &filename.contains("docx") | &filename.contains("odt") {
            let parsed_document = parse(extract(Document{path: String::from(path.to_string_lossy()), filename: filename.to_string(), temp_dir: None, emails: None}));
            let mut out_file = OpenOptions::new()
                .write(true)
                .append(true)
                .open("emails.txt")
                .unwrap();
            for email in parsed_document.emails.unwrap() {
              if let Err(e) = writeln!(out_file, "{}", email) {
                  eprintln!("Couldn't write to file: {}", e);
              }
            }
        }
    }
}

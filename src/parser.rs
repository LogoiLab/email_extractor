use crate::extractor::Document;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use regex::Regex;

pub fn parse(document: Document) -> Document {
    let temp_dir = document.temp_dir;
    let path = document.path;
    let filename = document.filename;
    let f = match File::open(temp_dir.unwrap().path().join(&filename)) {
        Ok(o) => Some(o),
        Err(e) => {
            println!("{}", e);
            None
        },
    };
    let file = BufReader::new(f.unwrap());
    let mut matches: Vec<String> = vec!();
    for line in file.lines() {
        match line {
            Ok(e) =>  {
                for mat in Regex::new(r"/^[a-zA-Z0-9.!#$%&’*+/=?^_`{|}~-]+@[a-zA-Z0-9-]+(?:\.[a-zA-Z0-9-]+)*$/").unwrap().find_iter(e.as_str()) {
                    matches.push(mat.as_str().to_string());
                }
            },
            Err(_) => {},
        };
    }
    Document{path: path, filename: filename, temp_dir: None, emails: Some(matches)}
}

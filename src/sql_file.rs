use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;

use std::path::PathBuf;

pub struct SqlFile {
    pub table_name: String,
    pub rows: i32,
    pub sets: i32,
    pub maximum_rows_per_query: i32
}

pub struct NodeFile {
    pub sql:SqlFile,
    pub file: File
}

impl NodeFile {
    pub fn new(output_dir:String,maximum_rows_per_query:i32)->NodeFile{
        let file_name = String::from("nodes.sql");

        let file_path = PathBuf::from(output_dir).join(file_name);

        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create_new(true)
            .open(file_path)
            .unwrap();

        if let Err(e) = writeln!(file, "DROP TABLE IF EXISTS nodes;") {
            drop(file);
            panic!("Could not to write to the file: {:?}",e);
        }

        if let Err(e) = writeln!(file, "CREATE TABLE nodes ();") {
            drop(file);
            panic!("Could not to write to the file: {:?}",e);
        }

        return NodeFile {
            sql: SqlFile {
                table_name: String::from("nodes"),
                rows: 0,
                sets: 0,
                maximum_rows_per_query:maximum_rows_per_query
            },
            file: file
        }
    }
}


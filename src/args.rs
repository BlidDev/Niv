use std::{io::{BufReader, BufRead}, path::Path, ffi::OsStr, fs::File};

use clap::Parser;

use crate::{structs::{ERROR, GError}, gerr};


#[derive(Debug, Parser)]
pub struct Arguments {
    #[clap(long, short, about)]
    /// A list of *.glg files to be inserted into memory in order
    #[arg(num_args(0..))]
    file_list : Option<Vec<String>>,
    #[clap(long,short,about)]
    /// A single  *.prj file that manages your project's sturcture
    project_file : Option<String>
}


impl Arguments {
   
    pub fn args_to_lines(&self) -> Result<Vec<String>, ERROR> {

        match (&self.file_list, &self.project_file) {
            (Some(v), None) => read_file_list(v),
            (None, Some(n)) => read_project_file(n),
            (None, None) => read_file_list(&vec!["main.glg".to_string()]),
            (Some(_), Some(_)) => gerr!("Error: cannot handle file_list and project_file at onces, use only one of the two")
        }
    }
}


fn read_file_list(list : &Vec<String>) -> Result<Vec<String>, ERROR> {

    let mut lines = vec![];

    for filename in list {
        let path = Path::new(filename);
        if path.extension().and_then(OsStr::to_str) != Some("glg") || !path.exists(){
            return gerr!("Error: invalid *.glg file name [{}]", filename);
        }
        let reader = BufReader::new(File::open(filename)?);
        for line in reader.lines() {
            let line = line?;
            let line = line.replace("(", "[").replace(")", "]");
            lines.push(line.trim().to_string());
        }
        
    }

    Ok(lines)
}


fn read_project_file(name : &String) -> Result<Vec<String>, ERROR> {
    let path = Path::new(name);

    if path.extension().and_then(OsStr::to_str) != Some("prj") || !path.exists() {
        return gerr!("Error: invalid *.prj file name [{}]", name);
    }

    let addition = match get_addition(path) {

        Some (s) => {
            let mut add = s;
            if !add.is_empty() {
                add.push('/');
            }
            add
        },
        None => "".to_string()
    };



    let mut files = vec![];
    let reader = BufReader::new(File::open(name)?);
    for line in reader.lines() {
        let mut add = addition.clone();
        add.push_str(&line?);
        files.push(add);
    }

    read_file_list(&files)
}

fn get_addition(path : &Path) -> Option<String> {
    let Some(addition) = path.parent() else {
        return None;
    };

    Some(
        addition
        .to_str()
        .expect(&format!("Error: could not convert filepath: [{:?}] to string", addition))
        .to_string()
    )
}

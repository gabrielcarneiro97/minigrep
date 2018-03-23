mod data_types;
mod finder;

use std::fs::File;
use std::io::prelude::*;
use std::env::Args;

use self::finder::find_all;
use self::data_types::DataCoord;

pub struct Grep {
    query: String,
    filename: String,
    contents: Vec<String>,
    result: Vec<DataCoord>,
}

impl Grep {
    pub fn new(mut args: Args) -> Grep {

        args.next();

        let query = match args.next() {
            Some(v) => v,
            None => panic!("Query string not found!"),
        };

        let filename = match args.next() {
            Some(v) => v,
            None => panic!("Filename string not found!")
        };

        let mut this = Grep {
            query: query,
            filename: filename,
            contents: Vec::new(),
            result: Vec::new()
        };

        this.read_file();

        println!("\n\t GREP\n\t looking for '{}' at '{}'\n", &this.query, &this.filename);

        this
    }
    pub fn read_file (&mut self) {

        let filename = &self.filename;

        let mut f = File::open(filename).expect("file not found");;

        let mut contents = String::new();

        f.read_to_string(&mut contents).expect("something went wrong reading the file");

        let contents : Vec<String> = contents.split('\n').map(|s| s.to_string()).collect();
        
        self.contents = contents;

    }

    pub fn find_printing (&mut self) {
        let contents = &self.contents;

        let query = &self.query;

        let mut is_here : Vec<DataCoord> = Vec::new();   

        for (i, line) in contents.iter().enumerate() {
            let appear = find_all(line, &query, 0);
            for col in appear {
                is_here.push(DataCoord {
                    line: i,
                    col: col,
                });

                self.print_line(i, col);
            }
        }

        self.result = is_here;
    }

    pub fn find_data (&mut self) {

        let contents = &self.contents;

        let query = &self.query;

        let mut is_here : Vec<DataCoord> = Vec::new();   

        for (i, line) in contents.iter().enumerate() {
            let appear = find_all(line, &query, 0);
            for col in appear {
                is_here.push(DataCoord {
                    line: i,
                    col: col,
                });
            }
        }

        self.result = is_here;

    }

    pub fn print_result(&self) {
        let result = &self.result;

        for (_, coord) in result.iter().enumerate() {
            self.print_line(coord.line, coord.col);
        }
    }

    fn print_line(&self, line : usize, col : usize) {
        let mut space_line = String::new();

            let chars = self.contents[line].chars();

            for (i, _) in chars.enumerate() {
                if i == col {
                    space_line.push('^');
                } else {
                    space_line.push('-');
                }
            }

            let pre_data = format!("({}:{}):", line + 1, col + 1);

            let pre_space : String = pre_data.chars()
                .map(|x| match x { _ => ' '}).collect();

            println!("{} {}", pre_data, self.contents[line]);
            println!("{} {}", pre_space, space_line);
    }
}
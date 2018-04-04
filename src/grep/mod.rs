mod data_types;
mod finder;

use std::fs::File;
use std::io::prelude::*;
use std::env::Args;

use self::finder::find_all;
use self::data_types::DataCoord;

/// Struct that contains the data.
pub struct Grep {
    query: String,
    filename: String,
    contents: Vec<String>,
    result: Vec<DataCoord>,
}

impl Grep {
    /// Method that recives the arguments from the enviroment and creates the Grep.
    /// 
    /// # Example
    /// ```
    /// let mut grep = Grep::new(env::args());
    /// ```
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
    /// Private method that reads the file from Grep.filename and converts it to a Vec<Strings>, each String has one line of the file.
    fn read_file (&mut self) {

        let filename = &self.filename;

        let mut f = File::open(filename).expect("file not found");;

        let mut contents = String::new();

        f.read_to_string(&mut contents).expect("something went wrong reading the file");

        let contents : Vec<String> = contents.lines().map(|s| String::from(s)).collect();
        
        self.contents = contents;

    }

    /// Method that search for the query and prints the line immediately after find, if finds. 
    pub fn find_printing (&mut self) {
        let contents = &self.contents;

        let query = &self.query;

        let mut is_here : Vec<DataCoord> = Vec::new();   

        for (i, line) in contents.iter().enumerate() {
            let appear = find_all(line, &query);
            let mut counter = 0;
            for col in appear {
                is_here.push(DataCoord {
                    line: i,
                    col: col,
                });

                let sub = (query.len() - 1) as i32 * counter;
                counter += 1;


                if col as i32 - sub as i32 >= 0 {
                    self.print_line(i, col - sub as usize);                    
                } else {
                    self.print_line(i, 0);
                }
                
            }
        }

        self.result = is_here;
    }

    /// Search for the query on the content and saves the result at result attribute.
    pub fn find_data (&mut self) {

        let contents = &self.contents;

        let query = &self.query;

        let mut is_here : Vec<DataCoord> = Vec::new();   

        for (i, line) in contents.iter().enumerate() {
            let appear = find_all(line, &query);
            for col in appear {
                is_here.push(DataCoord {
                    line: i,
                    col: col,
                });
            }
        }

        self.result = is_here;

    }

    /// Prints the result attribute formated.
    pub fn print_result(&self) {
        let result = &self.result;

        for (_, coord) in result.iter().enumerate() {
            self.print_line(coord.line, coord.col);
        }
    }

    /// Private method to print one line.
    fn print_line(&self, line : usize, col : usize) {
        let mut space_line = String::new();

            let chars = self.contents[line].char_indices();

            for (i, c) in chars.enumerate() {

                if i == col {
                    space_line.push('^');
                } else {
                    space_line.push('-');
                }

                // println!("{:?}, {}", c, col);
            }

            let pre_data = format!("({}:{}):", line + 1, col + 1);

            let pre_space : String = pre_data.chars()
                .map(|x| match x { _ => ' '}).collect();

            println!("{} {}", pre_data, self.contents[line]);
            println!("{} {}", pre_space, space_line);
    }
}
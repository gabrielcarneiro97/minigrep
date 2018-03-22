use std::env;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
#[derive(Clone)]
struct DataCoord {
    line: usize,
    col: usize,
}

fn find(s: &str, query: &str) -> Option<usize> {

    let string = s.to_string();
    let query = query.to_string();

    let string = string.chars().enumerate();
    let mut query_chars = query.chars();

    let mut q_ch = query_chars.next();

    let mut beg : Option<usize> = None;

    for (i, ch) in string {
        if beg != None && Some(ch) != q_ch {
            beg = None;
            query_chars = query.chars();
            q_ch = query_chars.next();
        } else if Some(ch) == q_ch {
            if beg == None {
                beg = Some(i);
            }
            q_ch = query_chars.next();

            if q_ch == None {
                return beg;
            }
        }
    }

    None
}

fn find_all(s: &str, query: &str, beg: usize) -> Vec<usize> {
    let mut ret : Vec<usize> = Vec::new();

    let string = s.to_string();

    match find(s, query) {
        Some(d) => {
            ret.push(d + beg);
            match string.find(query) {
                Some(t) => {
                    ret.append(&mut find_all(&s[t+1..], query, d+1+beg));
                    ret
                },
                None => ret
            }
        },
        None => ret
    }
}

struct Grep {
    query: String,
    filename: String,
    contents: Vec<String>,
    result: Vec<DataCoord>,
}

impl Grep {
    fn find(args: &[String]) -> Grep {
        let mut this = Grep {
            query: args[1].clone(),
            filename: args[2].clone(),
            contents: Vec::new(),
            result: Vec::new()
        };

        this.read_file();

        this
    }
    fn read_file (&mut self) {

        let filename = self.filename.clone();

        let mut f = File::open(filename).expect("file not found");;

        let mut contents = String::new();

        f.read_to_string(&mut contents).expect("something went wrong reading the file");

        let contents : Vec<String> = contents.split('\n').map(|s| s.to_string()).collect();
        
        self.contents = contents.clone();

    }

    fn find_data (&mut self) -> Vec<DataCoord> {

        let contents = self.contents.clone();

        let query = self.query.clone();

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

        self.result = is_here.clone();

        is_here
    }

    fn print_result(&self) {
        let result = self.result.clone();

        for (_, coord) in result.iter().enumerate() {
            let mut space_line = String::new();

            let chars = self.contents[coord.line].chars();

            for (i, _) in chars.enumerate() {
                if i == coord.col {
                    space_line.push('^');
                } else {
                    space_line.push('-');
                }
            }
            println!("({}:{}):", coord.line + 1, coord.col + 1);
            println!("{}", self.contents[coord.line]);
            println!("{}", space_line);

        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let mut grep = Grep::find(&args);

    grep.find_data();

    grep.print_result();

}

mod grep;

use std::env;

use grep::Grep;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let mut grep = Grep::new(&args);

    grep.find_printing();

}

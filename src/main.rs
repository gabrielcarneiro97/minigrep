mod grep;

use std::env;

use grep::Grep;

fn main() {
    let mut grep = Grep::new(env::args());

    grep.find_printing();

}

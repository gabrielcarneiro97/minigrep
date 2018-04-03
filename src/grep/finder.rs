/// Function that finds everytime that the query appears at string.
/// 
/// # Example
/// ```
/// let s = "cool!";
/// let index_of_o = find_all(s, "o", 0);
/// let expected : Vec<usize> = vec![1, 2];
/// 
/// assert_eq!(index_of_o, expected);
/// ```
pub fn find_all(s: &str, query: &str) -> Vec<usize> {
    let mut ret : Vec<usize> = Vec::new();

    let mut string = s.to_string();

    let mut hold : usize = 0;
    
    while !string.is_empty() {
        match string.find(query) {
            Some(id) => {
                ret.push(id + hold);
                let mut sum = 1;
                
                while !string.is_char_boundary(id+sum) {
                    sum += 1;
                }
                hold = id + sum + hold;
                string = string[id+sum..].to_string();

            }
            None => {
                string = String::new();
            }
        }
    }
    ret
}

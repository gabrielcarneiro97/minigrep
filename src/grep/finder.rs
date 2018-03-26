
/// Function that searches for a specific word in a str, it returns the index of first char.
/// 
/// # Example
/// ```
/// let s = "cool!";
/// let index_of_c = find(s, "c");
/// 
/// assert_eq!(index_of_c, 0);
/// ```
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
pub fn find_all(s: &str, query: &str, beg: usize) -> Vec<usize> {
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

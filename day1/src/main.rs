use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("input.txt") {
        let mut vec: Vec<i32> = Vec::new();

        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(num) = line {
                let n = num.parse::<i32>().unwrap();
                vec.push(n);
            }
        }

        let mut count = 0;
        for i in 1..vec.len() {
            if vec[i-1] < vec[i] {
                count += 1;
            }
        }

        let mut count2 = 0;
        for i in 3..vec.len() {
            if vec[i-3] < vec[i] {
                count2 += 1;
            }
        }

        println!("{}", count);
        println!("{}", count2);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

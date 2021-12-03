use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("input.txt") {
        let mut pos = 0;
        let mut depth = 0;
        let mut pos2 = 0;
        let mut depth2 = 0;
        let mut aim2 = 0;
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(l) = line {
                let vec: Vec<&str> = l.split(" ").collect();
                let n = vec[1].parse::<i32>().unwrap();
                match vec[0]{
                    "forward"=>{
                        pos+=n;
                        pos2+=n;
                        depth2+=n*aim2;
                    },
                    "up"=>{
                        depth-=n;
                        aim2-=n;
                    },
                    "down"=>{
                        depth+=n;
                        aim2+=n;
                    },
                    _=>println!("invalid"),
                }
            }
        }

        println!("{}", pos*depth);
        println!("{}", pos2*depth2);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

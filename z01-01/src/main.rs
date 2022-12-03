use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("/home/wiszowaty/personal/CodeAdvent2022/z01-01/src/input") {
        let mut max = 0;
        let mut actual = 0;

        for line_res in lines {
            if let Ok(line) = line_res {
                if line.len() == 0 {
                    if actual > max {
                        max = actual;
                    }
                    actual = 0;
                    continue;
                }
                let value : i32 = line.parse().unwrap();
                actual += value;
            }
        }

        if actual > max {
            max = actual;
        }

        print!("{}", max.to_string());
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

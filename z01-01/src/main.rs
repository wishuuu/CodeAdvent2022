use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("/home/wiszowaty/personal/CodeAdvent2022/z01-01/src/input") {
        let max: &mut [i32; 3] = &mut [0, 0, 0];
        let mut actual = 0;

        for line_res in lines {
            if let Ok(line) = line_res {
                if line.len() == 0 {
                    if actual > max[0] {
                        max[0] = actual;
                        sort(max);
                    }
                    actual = 0;
                    continue;
                }
                let value: i32 = line.parse().unwrap();
                actual += value;
            }
        }

        if max[0] < actual {
            max[0] = actual;
        } else if max[1] < actual {
            max[1] = actual;
        } else if max[2] < actual {
            max[2] = actual;
        }

        print!("{}", (max[0] + max[1] + max[2]).to_string());
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn sort(input: &mut [i32; 3]) {
    for i in 0..input.len() {
        for j in 0..input.len() {
            if input[i] < input[j] {
                let tmp = input[i];
                input[i] = input[j];
                input[j] = tmp;
            }
        }
    }
}
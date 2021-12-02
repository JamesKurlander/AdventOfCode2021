extern crate queues;

use queues::*;
use std::fs;
use std::env;

fn main() {
    let cwd = env::current_dir().unwrap();
    let mut path: String = cwd.as_os_str().to_str().unwrap().to_string();
    let filename = "/input.txt".to_string();
    path.push_str(&filename);
    let input = path;

    let contents = fs::read_to_string(input).expect("Something went wrong reading the file");
    let lines = contents.lines();

    let mut numLargerMeasurements = 0;
    let mut currentLine : i32 = 0;
    let mut removedLine : i32 = 0;
    let mut window : Queue<i32> = queue![];

    for line in lines {
        currentLine = line.parse::<i32>().unwrap();
        window.add(currentLine);

        if window.size() == 4 {
            removedLine = window.remove().unwrap();
            if(currentLine > removedLine) {
                numLargerMeasurements = numLargerMeasurements + 1;
            }
        }
    }
    
    println!("{}", numLargerMeasurements);
}

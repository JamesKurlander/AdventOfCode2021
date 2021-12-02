use std::fs;
use std::env;

fn main() {
    let mut numLargerMeasurements = 0;
    let mut previousLine: i32 = 0;
    let mut currentLine: i32 = 0;
    let cwd = env::current_dir().unwrap();
    let mut path: String = cwd.as_os_str().to_str().unwrap().to_string();
    let filename = "/input.txt".to_string();
    path.push_str(&filename);
    let input = path;
    //println!("{:?}", input);

    let contents = fs::read_to_string(input).expect("Something went wrong reading the file");
    //println!("{}", contents);

    let lines = contents.lines();

    for line in lines {
        currentLine = line.parse::<i32>().unwrap();
        if currentLine > previousLine {
            numLargerMeasurements = numLargerMeasurements + 1;
        }
        previousLine = currentLine;
    }

    //-1 to account for the first invalid larger measurement increment
    println!("{}", numLargerMeasurements - 1);
}

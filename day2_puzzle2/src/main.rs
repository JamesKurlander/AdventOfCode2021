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

    let mut vertical = 0;
    let mut horizontal = 0;
    let mut aim = 0;

    for line in lines {
        let movement: Vec<&str> = line.split(' ').collect();
        let distance = movement[1].parse::<i32>().unwrap();
        match movement[0] {
            "up"=>aim = aim - distance,
            "down"=>aim = aim + distance,
            "forward"=> { horizontal = horizontal + distance;
                          vertical = vertical + aim * distance },
            &_ => println!("Error"),
        };
    }

    println!("{}", vertical * horizontal);
}
